BEGIN TRANSACTION;

----------------------------------------------------------------------------------------------------------
-- Define static AvailableModels table

------------------------------------------------------------
-- SSH remote model entry example
LET $modelUID = <string> rand::uuid::v4();

CREATE type::thing("ConnTypeParams", $modelUID) CONTENT {
    uid: $modelUID,
    createdAt: time::now(),
    lastUpdated: time::now(),

    -- SSH connection configuration
    ip: "127.0.0.1",
    port: 6000,
    user: "admin",
    pass: "admin"
} RETURN uid;

CREATE type::thing("ModelParams", $modelUID) CONTENT {
    uid: $modelUID,
    createdAt: time::now(),
    lastUpdated: time::now(),

    -- Model parameters
    command: "./inference.py",
    modelPath: "/home/admin/models/model1",
} RETURN uid;

CREATE type::thing("AvailableModels", $modelUID) CONTENT {
    uid: $modelUID,
    createdAt: time::now(),
    lastUpdated: time::now(),

    -- Model parameters
    name: "model1",
    connType: "ssh",

    -- Dynamic connTypeParams table
    connTypeParams: array::add([], type::thing("ConnTypeParams", $modelUID)),

    -- Dynamic modelParams table
    modelParams: array::add([], type::thing("ModelParams", $modelUID))

} RETURN uid;
------------------------------------------------------------




COMMIT TRANSACTION;