use async_trait::async_trait;
use sqlx::SqlitePool;
use actix_web::Result;

#[derive(Clone, Debug)]
pub struct ActionIndicatorCheckMessage(pub bool, pub String);

#[async_trait]
pub trait IActionIndicator {
    fn get_name(&self) -> String;
    fn get_label(&self) -> String;
    fn get_description(&self) -> String;
    
    // name of the action this indicator applies to
    fn get_action_name(&self) -> String;
    
    // when to run / how often
    fn get_cron_schedule(&self) -> String;

    // the code to run to return if the action should be indicated or not
    async fn perform_indicator_check_action(&self, pool: &SqlitePool) -> Result<ActionIndicatorCheckMessage>;
}