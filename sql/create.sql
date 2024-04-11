create database backend;
use backend;
CREATE TABLE softwares (
    archive VARCHAR(255) NOT NULL,
    version VARCHAR(255) NOT NULL,
    component VARCHAR(255) ,
    origin VARCHAR(255) ,
    label VARCHAR(255) ,
    architecture VARCHAR(255) ,
    download VARCHAR(255) NOT NULL,
    others VARCHAR(255) ,
    flag BOOLEAN DEFAULT TRUE ,
    PRIMARY KEY (archive, version)
);

//
INSERT INTO softwares (archive, version, component, origin, label, architecture, download, others, flag)
VALUES ('example_archive', '1.0', 'example_component', 'example_origin', 'example_label', 'example_architecture', 'example_download', 'example_others', TRUE);
