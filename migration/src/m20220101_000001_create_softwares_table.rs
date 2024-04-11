use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Softwares::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Softwares::Archive)
                            .string()
                            .not_null()
                    )
                    .col(ColumnDef::new(Softwares::Version).string().not_null())
                    .primary_key(
                        Index::create()
                            .name("primary_index")
                            .col(Softwares::Archive)
                            .col(Softwares::Version)
                            .primary(),) // 定义复合主键
                    .col(ColumnDef::new(Softwares::VersionMajor).integer().not_null())
                    .col(ColumnDef::new(Softwares::VersionMinor).integer().not_null())
                    .col(ColumnDef::new(Softwares::VersionPatch).integer().not_null())
                    .col(ColumnDef::new(Softwares::Component).string())
                    .col(ColumnDef::new(Softwares::Origin).string())
                    .col(ColumnDef::new(Softwares::Label).string())
                    .col(ColumnDef::new(Softwares::Architecture).string())
                    .col(ColumnDef::new(Softwares::Download).string())
                    .col(ColumnDef::new(Softwares::Others).string())
                    .col(ColumnDef::new(Softwares::Flag).boolean())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Softwares::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Softwares {
    Table,
    Archive,
    Version,
    VersionMajor,
    VersionMinor,
    VersionPatch,
    Component,
    Origin,
    Label,
    Architecture,
    Download,
    Others,
    Flag,
}
