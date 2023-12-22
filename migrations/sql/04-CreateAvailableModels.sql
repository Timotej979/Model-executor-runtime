BEGIN TRANSACTION;

------------------------------------------------------------------------------------------

------------------------------------------------------------
-- SSH remote model entry example admin1-model1
LET $modelUID = <string> rand::uuid::v4();

CREATE type::thing("ConnTypeParams", $modelUID) CONTENT {
    uid: $modelUID,
    createdAt: time::now(),
    lastUpdated: time::now(),

    -- SSH connection configuration
    host: "127.0.0.1",
    port: 6000,
    user: "admin1",
    pass: "admin1",
} RETURN uid;

CREATE type::thing("ModelParams", $modelUID) CONTENT {
    uid: $modelUID,
    createdAt: time::now(),
    lastUpdated: time::now(),

    -- Model parameters
    command: "./inference.py",
    modelPath: "/home/admin1/models/model1",
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
    modelParams: array::add([], type::thing("ModelParams", $modelUID)),
} RETURN uid;
------------------------------------------------------------

------------------------------------------------------------
-- SSH remote model entry example admin1-model2
LET $modelUID = <string> rand::uuid::v4();

CREATE type::thing("ConnTypeParams", $modelUID) CONTENT {
    uid: $modelUID,
    createdAt: time::now(),
    lastUpdated: time::now(),

    -- SSH connection configuration
    host: "127.0.0.1",
    port: 6000,
    user: "admin1",
    pass: "admin1",
} RETURN uid;

CREATE type::thing("ModelParams", $modelUID) CONTENT {
    uid: $modelUID,
    createdAt: time::now(),
    lastUpdated: time::now(),

    -- Model parameters
    command: "./inference.py",
    modelPath: "/home/admin1/models/model2",
} RETURN uid;

CREATE type::thing("AvailableModels", $modelUID) CONTENT {
    uid: $modelUID,
    createdAt: time::now(),
    lastUpdated: time::now(),

    -- Model parameters
    name: "model2",
    connType: "ssh",

    -- Dynamic connTypeParams table
    connTypeParams: array::add([], type::thing("ConnTypeParams", $modelUID)),

    -- Dynamic modelParams table
    modelParams: array::add([], type::thing("ModelParams", $modelUID))
} RETURN uid;
------------------------------------------------------------


------------------------------------------------------------
-- SSH remote model entry example admin2-model1
LET $modelUID = <string> rand::uuid::v4();

CREATE type::thing("ConnTypeParams", $modelUID) CONTENT {
    uid: $modelUID,
    createdAt: time::now(),
    lastUpdated: time::now(),

    -- SSH connection configuration
    host: "127.0.0.1",
    port: 7000,
    user: "admin2",
    pass: "admin2",
} RETURN uid;

CREATE type::thing("ModelParams", $modelUID) CONTENT {
    uid: $modelUID,
    createdAt: time::now(),
    lastUpdated: time::now(),

    -- Model parameters
    command: "./inference.py",
    modelPath: "/home/admin2/models/model1",
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


------------------------------------------------------------
-- SSH remote model entry example admin2-1model2
LET $modelUID = <string> rand::uuid::v4();

CREATE type::thing("ConnTypeParams", $modelUID) CONTENT {
    uid: $modelUID,
    createdAt: time::now(),
    lastUpdated: time::now(),

    -- SSH connection configuration
    host: "127.0.0.1",
    port: 7000,
    user: "admin2",
    pass: "admin2",
} RETURN uid;

CREATE type::thing("ModelParams", $modelUID) CONTENT {
    uid: $modelUID,
    createdAt: time::now(),
    lastUpdated: time::now(),

    -- Model parameters
    command: "./inference.py",
    modelPath: "/home/admin2/models/model2",
} RETURN uid;

CREATE type::thing("AvailableModels", $modelUID) CONTENT {
    uid: $modelUID,
    createdAt: time::now(),
    lastUpdated: time::now(),

    -- Model parameters
    name: "model2",
    connType: "ssh",

    -- Dynamic connTypeParams table
    connTypeParams: array::add([], type::thing("ConnTypeParams", $modelUID)),

    -- Dynamic modelParams table
    modelParams: array::add([], type::thing("ModelParams", $modelUID))
} RETURN uid;
------------------------------------------------------------

------------------------------------------------------------------------------------------

------------------------------------------------------------
-- Local model entry example
LET $modelUID = <string> rand::uuid::v4();

CREATE type::thing("ConnTypeParams", $modelUID) CONTENT {
    uid: $modelUID,
    createdAt: time::now(),
    lastUpdated: time::now(),

    -- Local connection configuration if any (empty)
} RETURN uid;

CREATE type::thing("ModelParams", $modelUID) CONTENT {
    uid: $modelUID,
    createdAt: time::now(),
    lastUpdated: time::now(),

    -- Model parameters
    command: "./inference.py",
    modelPath: "/home/models/model1",
} RETURN uid;

CREATE type::thing("AvailableModels", $modelUID) CONTENT {
    uid: $modelUID,
    createdAt: time::now(),
    lastUpdated: time::now(),

    -- Model parameters
    name: "model1",
    connType: "local",

    -- Dynamic connTypeParams table
    connTypeParams: array::add([], type::thing("ConnTypeParams", $modelUID)),

    -- Dynamic modelParams table
    modelParams: array::add([], type::thing("ModelParams", $modelUID))
} RETURN uid;
------------------------------------------------------------


------------------------------------------------------------
-- Local model entry example
LET $modelUID = <string> rand::uuid::v4();

CREATE type::thing("ConnTypeParams", $modelUID) CONTENT {
    uid: $modelUID,
    createdAt: time::now(),
    lastUpdated: time::now(),

    -- Local connection configuration if any (empty)
} RETURN uid;

CREATE type::thing("ModelParams", $modelUID) CONTENT {
    uid: $modelUID,
    createdAt: time::now(),
    lastUpdated: time::now(),

    -- Model parameters
    command: "./inference.py",
    modelPath: "/home/models/model2",
} RETURN uid;

CREATE type::thing("AvailableModels", $modelUID) CONTENT {
    uid: $modelUID,
    createdAt: time::now(),
    lastUpdated: time::now(),

    -- Model parameters
    name: "model2",
    connType: "local",

    -- Dynamic connTypeParams table
    connTypeParams: array::add([], type::thing("ConnTypeParams", $modelUID)),

    -- Dynamic modelParams table
    modelParams: array::add([], type::thing("ModelParams", $modelUID))
} RETURN uid;
------------------------------------------------------------

------------------------------------------------------------------------------------------

COMMIT TRANSACTION;