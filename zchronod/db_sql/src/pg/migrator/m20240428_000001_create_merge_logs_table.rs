use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m_20240428_000001_create_merge_logs_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    // Define how to apply this migration: Create the Mergelogs table.
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(MergeLogs::Table)
                    .col(
                        ColumnDef::new(MergeLogs::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(MergeLogs::FromId).char_len(32).not_null())
                    .col(ColumnDef::new(MergeLogs::ToId).char_len(32).not_null())
                    .col(ColumnDef::new(MergeLogs::StartCount).integer().not_null())
                    .col(ColumnDef::new(MergeLogs::EndCount).integer().not_null())
                    .col(ColumnDef::new(MergeLogs::SClockHash).char_len(64).not_null())
                    .col(ColumnDef::new(MergeLogs::EClockHash).char_len(64).not_null())
                    .col(ColumnDef::new(MergeLogs::MergeAt).timestamp().not_null())
                    .to_owned(),
            )
            .await
    }

    // Define how to rollback this migration: Drop the Chef table.
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(MergeLogs::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum MergeLogs {
    Table,
    Id,
    FromId,
    ToId,
    StartCount,
    EndCount,
    SClockHash,
    EClockHash,
    MergeAt
}