

use tonic::{Request, Response, Status};
use query::query_server::Query;
use query::{ReadLogsRequest, ReadLogsResponse, LogEntry};
use datafusion::prelude::*;
use chrono::DateTime;
// use chrono::offset::Utc;

pub mod query {
    tonic::include_proto!("query");
}

#[derive(Debug, Default)]
pub struct QueryService;

#[tonic::async_trait]
impl Query for QueryService {
    async fn read_logs(
        &self,
        request: Request<ReadLogsRequest>,
    ) -> Result<Response<ReadLogsResponse>, Status> {
        let req = request.into_inner();

        // Parse the date_from and date_to from the request
        let date_from = DateTime::parse_from_rfc3339(&req.date_from)
            .map_err(|_| Status::invalid_argument("Invalid date_from"))?;
        let date_to = DateTime::parse_from_rfc3339(&req.date_to)
            .map_err(|_| Status::invalid_argument("Invalid date_to"))?;

        // Create a DataFusion execution context
        let ctx = SessionContext::new();

        // Register the Parquet file as a table
        ctx.register_parquet("logs", "logs.parquet", ParquetReadOptions::default())
            .await
            .map_err(|e| Status::internal(format!("Failed to register parquet: {:?}", e)))?;

        // Build a SQL query to filter by tenant and date range
        let sql = format!(
            "SELECT datetime, tenant_name, item_id, status, qty FROM logs WHERE tenant_name = '{}' AND datetime >= '{}' AND datetime <= '{}'",
            req.tenant, date_from, date_to
        );

        // Execute the query
        let df = ctx
            .sql(&sql)
            .await
            .map_err(|e| Status::internal(format!("Failed to execute query: {:?}", e)))?;

        // Collect the query results into a vector of log entries
        let batches = df.collect().await.map_err(|e| {
            Status::internal(format!("Failed to collect query results: {:?}", e))
        })?;

        // Convert Arrow record batches to gRPC LogEntry messages
        let mut logs = vec![];
        for batch in batches {
            let columns = batch.columns();

            let datetimes = columns[0].as_any().downcast_ref::<arrow::array::StringArray>().unwrap();
            let tenant_names = columns[1].as_any().downcast_ref::<arrow::array::StringArray>().unwrap();
            let item_ids = columns[2].as_any().downcast_ref::<arrow::array::StringArray>().unwrap();
            let statuses = columns[3].as_any().downcast_ref::<arrow::array::StringArray>().unwrap();
            let qtys = columns[4].as_any().downcast_ref::<arrow::array::Float64Array>().unwrap();

            for i in 0..batch.num_rows() {
                logs.push(LogEntry {
                    datetime: datetimes.value(i).to_string(),
                    tenant_name: tenant_names.value(i).to_string(),
                    item_id: item_ids.value(i).to_string(),
                    status: statuses.value(i).to_string(),
                    qty: qtys.value(i),
                });
            }
        }

        let response = ReadLogsResponse { logs };
        Ok(Response::new(response))
    }
}

