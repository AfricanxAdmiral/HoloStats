#[path = "../error.rs"]
mod error;
#[path = "../requests/mod.rs"]
mod requests;
#[path = "../vtubers.rs"]
mod vtubers;

use chrono::Utc;
use sqlx::PgPool;
use std::env;

use crate::error::Result;
use crate::vtubers::VTUBERS;

#[tokio::main]
async fn main() -> Result<()> {
    let client = reqwest::Client::new();

    let pool = PgPool::connect(&env::var("DATABASE_URL").unwrap()).await?;

    let now = Utc::now();

    let ids = VTUBERS
        .iter()
        .filter_map(|v| v.bilibili)
        .collect::<Vec<_>>();

    let bilibili_channels = requests::bilibili_channels(&client, ids).await?;

    for channel in &bilibili_channels {
        if let Some(vtb) = VTUBERS.iter().find(|v| v.bilibili == Some(&channel.id)) {
            let _ = sqlx::query!(
                r#"
                    update bilibili_channels
                       set (subscriber_count, view_count, updated_at)
                         = ($1, $2, $3)
                     where vtuber_id = $4
                "#,
                channel.subscriber_count,
                channel.view_count,
                now,
                vtb.id,
            )
            .execute(&pool)
            .await?;

            let _ = sqlx::query!(
                r#"
                    insert into bilibili_channel_subscriber_statistic (vtuber_id, time, value)
                         values ($1, $2, $3)
                "#,
                vtb.id,
                now,
                channel.subscriber_count,
            )
            .execute(&pool)
            .await?;

            let _ = sqlx::query!(
                r#"
                    insert into bilibili_channel_view_statistic (vtuber_id, time, value)
                         values ($1, $2, $3)
                "#,
                vtb.id,
                now,
                channel.view_count,
            )
            .execute(&pool)
            .await?;
        }
    }

    let ids = VTUBERS.iter().filter_map(|v| v.youtube).collect::<Vec<_>>();

    let youtube_channels = requests::youtube_channels(&client, ids).await?;

    for channel in &youtube_channels {
        if let Some(vtb) = VTUBERS.iter().find(|v| v.youtube == Some(&channel.id)) {
            let _ = sqlx::query!(
                r#"
                    update youtube_channels
                       set (subscriber_count, view_count, updated_at)
                         = ($1, $2, $3)
                     where vtuber_id = $4
                "#,
                channel.subscriber_count,
                channel.view_count,
                now,
                vtb.id,
            )
            .execute(&pool)
            .await?;

            let _ = sqlx::query!(
                r#"
                    insert into youtube_channel_subscriber_statistic (vtuber_id, time, value)
                         values ($1, $2, $3)
                "#,
                vtb.id,
                now,
                channel.subscriber_count,
            )
            .execute(&pool)
            .await?;

            let _ = sqlx::query!(
                r#"
                    insert into youtube_channel_view_statistic (vtuber_id, time, value)
                         values ($1, $2, $3)
                "#,
                vtb.id,
                now,
                channel.view_count,
            )
            .execute(&pool)
            .await?;
        }
    }

    println!(
        "Bilibili Channels Uppdated: {} YouTube Channels Updated: {}",
        bilibili_channels.len(),
        youtube_channels.len()
    );

    Ok(())
}
