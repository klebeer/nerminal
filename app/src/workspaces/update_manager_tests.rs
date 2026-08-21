use warpui::{AddSingletonModel, App};

use super::*;
use crate::ai::credit_availability::{AICreditAvailability, AICreditDenialReason};
use crate::auth::AuthManager;
use crate::cloud_object::model::actions::ObjectActions;
use crate::cloud_object::model::persistence::CloudModel;
use crate::server::server_api::team::MockTeamClient;
use crate::server::server_api::workspace::{MockWorkspaceClient, WorkspaceClient};
use crate::server::sync_queue::SyncQueue;
use crate::server::telemetry::context_provider::AppTelemetryContextProvider;
use crate::settings::PrivacySettings;
use crate::system::SystemStats;
use crate::workspaces::user_profiles::UserProfiles;
use crate::workspaces::workspace::{PurchaseAddOnCreditsPolicy, Workspace};

fn initialize_app(
    team_client: Arc<dyn TeamClient>,
    workspace_client: Arc<dyn WorkspaceClient>,
    workspaces: Vec<Workspace>,
    app: &mut App,
) {
    app.add_singleton_model(|_| NetworkStatus::new());
    app.add_singleton_model(|_| SystemStats::new());
    app.add_singleton_model(TeamTesterStatus::new);
    app.add_singleton_model(|ctx| {
        UserWorkspaces::mock(
            team_client.clone(),
            workspace_client.clone(),
            workspaces,
            ctx,
        )
    });
    app.add_singleton_model(SyncQueue::mock);
    app.add_singleton_model(CloudModel::mock);
    app.add_singleton_model(|_| ObjectActions::new(vec![]));
    app.add_singleton_model(PrivacySettings::mock);
    app.add_singleton_model(|_| UserProfiles::new(vec![]));
    app.add_singleton_model(|_| ServerApiProvider::new_for_test());
    app.add_singleton_model(|_| AuthStateProvider::new_for_test());
    app.add_singleton_model(AppTelemetryContextProvider::new_context_provider);
    app.add_singleton_model(AuthManager::new_for_test);
}

#[test]
fn test_workspace_metadata_piggyback_feeds_ai_credit_availability() {
    App::test((), |mut app| async move {
        let team_client = Arc::new(MockTeamClient::new());
        initialize_app(
            team_client.clone(),
            Arc::new(MockWorkspaceClient::new()),
            vec![],
            &mut app,
        );
        if app
            .models_of_type::<settings::PrivatePreferences>()
            .is_empty()
        {
            app.update(crate::settings::init_and_register_user_preferences);
        }
        app.add_singleton_model(|ctx| {
            AIRequestUsageModel::new_for_test(ServerApiProvider::as_ref(ctx).get_ai_client(), ctx)
        });
        let team_update_manager =
            app.add_singleton_model(|ctx| TeamUpdateManager::new(team_client, None, ctx));

        let availability = AICreditAvailability::unavailable(AICreditDenialReason::OutOfCredits);
        team_update_manager.update(&mut app, |manager, ctx| {
            manager.on_workspaces_updated(
                Ok(WorkspacesMetadataResponse {
                    workspaces: vec![],
                    joinable_teams: vec![],
                    experiments: None,
                    feature_model_choices: None,
                    ai_credit_availability: Some(availability),
                    user_purchase_policy: None,
                }),
                ctx,
            );
        });

        AIRequestUsageModel::handle(&app).read(&app, |model, _| {
            assert_eq!(model.server_availability(), Some(availability));
        });
    });
}

#[test]
fn test_poll_path_apply_refreshes_user_purchase_policy() {
    App::test((), |mut app| async move {
        let team_client = Arc::new(MockTeamClient::new());
        let workspace_client = Arc::new(MockWorkspaceClient::new());
        initialize_app(team_client.clone(), workspace_client, vec![], &mut app);

        let team_update_manager =
            app.add_singleton_model(|ctx| TeamUpdateManager::new(team_client, None, ctx));

        // The periodic poll applies metadata through TeamUpdateManager's
        // own on_workspaces_updated; it must refresh the stored user-level
        // policy.
        let response_with_policy = WorkspacesMetadataResponse {
            workspaces: vec![],
            joinable_teams: vec![],
            experiments: None,
            feature_model_choices: None,
            ai_credit_availability: None,
            user_purchase_policy: Some(PurchaseAddOnCreditsPolicy {
                enabled: false,
                premium_enabled: true,
                price_premium_bps: 1000,
            }),
        };
        team_update_manager.update(&mut app, |manager, ctx| {
            manager.on_workspaces_updated(Ok(response_with_policy), ctx);
        });
        app.read(|ctx| {
            assert!(
                UserWorkspaces::as_ref(ctx)
                    .purchase_policy()
                    .is_some_and(|policy| policy.allows_purchases()),
                "a poll-path apply should store the user-level policy"
            );
        });

        // A later poll without the policy must clear the stored fallback so
        // it can't go stale.
        let response_without_policy = WorkspacesMetadataResponse {
            workspaces: vec![],
            joinable_teams: vec![],
            experiments: None,
            feature_model_choices: None,
            ai_credit_availability: None,
            user_purchase_policy: None,
        };
        team_update_manager.update(&mut app, |manager, ctx| {
            manager.on_workspaces_updated(Ok(response_without_policy), ctx);
        });
        app.read(|ctx| {
            assert!(
                UserWorkspaces::as_ref(ctx).purchase_policy().is_none(),
                "a poll-path apply without the policy should clear the stored fallback"
            );
        });
    });
}
