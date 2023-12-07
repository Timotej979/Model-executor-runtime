BEGIN TRANSACTION;

----------------------------------------------------------------------------------------------------------
-- Define static AvailableModels table

CREATE connTypeParams CONTENT {
    uid: rand::uuid::v4(),
    createdAt: datetime::now(),
    lastUpdated: datetime::now(),

    -- SSH connection configuration
    ip: "127.0.0.1",
    port: 6000,
    username: "admin"
    password: "admin"
} RETURN uid;

CREATE modelParams CONTENT {
    uid: rand::uuid::v4(),
    createdAt: datetime::now(),
    lastUpdated: datetime::now(),

    -- Model parameters
    command: "./inference.py",
    modelPath: "/home/admin/models/model1",
} RETURN uid;

CREATE AvailableModels CONTENT {
    uid: rand::uuid::v4(),
    createdAt: datetime::now(),
    lastUpdated: datetime::now(),

    -- Model parameters
    name: "model1",
    connType: "ssh",

    -- Dynamic connTypeParams table
    connTypeParams: [
        connTypeParams
    ],

    -- Dynamic modelParams table
    modelParams: [
        modelParams
    ]
} RETURN uid;



COMMIT TRANSACTION;