use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use tracing::Instrument;
use uuid::Uuid;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    let request_id = Uuid::new_v4();

    let request_span = tracing::info_span!(
        "Adding a new subscriber",
        %request_id,
        subscriber_email = %form.email,
        subscriber_name = %form.name
    );

    let _req_span_guard = request_span.enter();

    let query_span = tracing::info_span!("Saving details into database");

    tracing::info!(
        "Req Id: {} - Adding '{}' '{}' as a new sub.",
        request_id,
        form.email,
        form.name
    );
    tracing::info!(
        "Req Id: {} - Saving new subscriber details in the database.",
        request_id
    );
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool.get_ref())
    .instrument(query_span)
    .await
    {
        Ok(_) => {
            tracing::info!("Req Id: {} - Subscriber details saved.", request_id);
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!("Req Id: {} - Failed to execute query: {:?}", request_id, e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
