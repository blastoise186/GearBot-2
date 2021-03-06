use crate::core::CommandContext;
use crate::error::CommandResult;
use crate::gearbot_important;

pub async fn restart(ctx: CommandContext) -> CommandResult {
    ctx.bot_context
        .http
        .create_message(ctx.message.channel.get_id())
        .content("Shutting down")?
        .await?;

    gearbot_important!("Reboot initiated by {}", ctx.message.author.username);
    ctx.bot_context.initiate_cold_resume().await.unwrap();
    Ok(())
}
