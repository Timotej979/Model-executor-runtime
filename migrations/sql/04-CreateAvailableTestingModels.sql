BEGIN TRANSACTION;

------------------------------------------------------------------------------------------------------------------------------

------------------------------------------------------------
-- SSH remote model entry test example 1 (DialoGPT-small)
LET $modelUID = <string> rand::uuid::v4();

CREATE type::thing("ConnTypeParams", $modelUID) CONTENT {
    uid: $modelUID,
    createdAt: time::now(),
    lastUpdated: time::now(),

    -- SSH connection configuration
    host: "127.0.0.1",
    port: 2222,
    user: "admin",
    pass: "admin",
} RETURN uid;

CREATE type::thing("ModelParams", $modelUID) CONTENT {
    uid: $modelUID,
    createdAt: time::now(),
    lastUpdated: time::now(),

    -- Model parameters
    inferenceCommand: "python3 inference.py",
    trainCommand: "python3 train.py",
    modelPath: "/models/DialoGPT-small",
} RETURN uid;

CREATE type::thing("AvailableModels", $modelUID) CONTENT {
    uid: $modelUID,
    createdAt: time::now(),
    lastUpdated: time::now(),

    -- Model parameters
    name: "DialoGPT-small",
    connType: "ssh",

    -- Dynamic connTypeParams table
    connTypeParams: array::add([], type::thing("ConnTypeParams", $modelUID)),

    -- Dynamic modelParams table
    modelParams: array::add([], type::thing("ModelParams", $modelUID)),
} RETURN uid;
------------------------------------------------------------

------------------------------------------------------------
-- SSH remote model entry test example 2 (DialoGPT-medium)
LET $modelUID = <string> rand::uuid::v4();

CREATE type::thing("ConnTypeParams", $modelUID) CONTENT {
    uid: $modelUID,
    createdAt: time::now(),
    lastUpdated: time::now(),

    -- SSH connection configuration
    host: "127.0.0.1",
    port: 2222,
    user: "admin",
    pass: "admin",
} RETURN uid;

CREATE type::thing("ModelParams", $modelUID) CONTENT {
    uid: $modelUID,
    createdAt: time::now(),
    lastUpdated: time::now(),

    -- Model parameters
    inferenceCommand: "python3 inference.py",
    trainCommand: "python3 train.py",
    modelPath: "/models/DialoGPT-medium",
} RETURN uid;

CREATE type::thing("AvailableModels", $modelUID) CONTENT {
    uid: $modelUID,
    createdAt: time::now(),
    lastUpdated: time::now(),

    -- Model parameters
    name: "DialoGPT-medium",
    connType: "ssh",

    -- Dynamic connTypeParams table
    connTypeParams: array::add([], type::thing("ConnTypeParams", $modelUID)),

    -- Dynamic modelParams table
    modelParams: array::add([], type::thing("ModelParams", $modelUID))
} RETURN uid;
------------------------------------------------------------


------------------------------------------------------------
-- SSH remote model entry test example 3 (DialoGPT-large)
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
    inferenceCommand: "python3 inference.py",
    trainCommand: "python3 train.py",
    modelPath: "/models/DialoGPT-large",
} RETURN uid;

CREATE type::thing("AvailableModels", $modelUID) CONTENT {
    uid: $modelUID,
    createdAt: time::now(),
    lastUpdated: time::now(),

    -- Model parameters
    name: "DialoGPT-large",
    connType: "ssh",

    -- Dynamic connTypeParams table
    connTypeParams: array::add([], type::thing("ConnTypeParams", $modelUID)),

    -- Dynamic modelParams table
    modelParams: array::add([], type::thing("ModelParams", $modelUID))
} RETURN uid;
------------------------------------------------------------


------------------------------------------------------------------------------------------------------------------------------
------------------------------------------------------------------------------------------------------------------------------


------------------------------------------------------------
-- Local model entry test example 1 (DialoGPT-small)
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
    inferenceCommand: "conda activate transformer-venv && python3 inference.py",
    trainCommand: "python3 train.py",
    modelPath: "/home/timotej/Documents/GitProjects/Model-executor-runtime/test-models/local/DialoGPT-small",
} RETURN uid;

CREATE type::thing("AvailableModels", $modelUID) CONTENT {
    uid: $modelUID,
    createdAt: time::now(),
    lastUpdated: time::now(),

    -- Model parameters
    name: "DialoGPT-small",
    connType: "local",

    -- Dynamic connTypeParams table
    connTypeParams: array::add([], type::thing("ConnTypeParams", $modelUID)),

    -- Dynamic modelParams table
    modelParams: array::add([], type::thing("ModelParams", $modelUID))
} RETURN uid;
------------------------------------------------------------


------------------------------------------------------------
-- Local model entry test example 2 (DialoGPT-medium)
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
    inferenceCommand: "conda activate transformer-venv && python3 inference.py",
    trainCommand: "python3 train.py",
    modelPath: "/home/timotej/Documents/GitProjects/Model-executor-runtime/test-models/local/DialoGPT-medium",
} RETURN uid;

CREATE type::thing("AvailableModels", $modelUID) CONTENT {
    uid: $modelUID,
    createdAt: time::now(),
    lastUpdated: time::now(),

    -- Model parameters
    name: "DialoGPT-medium",
    connType: "local",

    -- Dynamic connTypeParams table
    connTypeParams: array::add([], type::thing("ConnTypeParams", $modelUID)),

    -- Dynamic modelParams table
    modelParams: array::add([], type::thing("ModelParams", $modelUID))
} RETURN uid;
------------------------------------------------------------

------------------------------------------------------------
-- Local model entry test example 3 (DialoGPT-large)
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
    inferenceCommand: "conda activate transformer-venv && python3 inference.py",
    trainCommand: "python3 train.py",
    modelPath: "/home/timotej/Documents/GitProjects/Model-executor-runtime/test-models/local/DialoGPT-large",
} RETURN uid;

CREATE type::thing("AvailableModels", $modelUID) CONTENT {
    uid: $modelUID,
    createdAt: time::now(),
    lastUpdated: time::now(),

    -- Model parameters
    name: "DialoGPT-large",
    connType: "local",

    -- Dynamic connTypeParams table
    connTypeParams: array::add([], type::thing("ConnTypeParams", $modelUID)),

    -- Dynamic modelParams table
    modelParams: array::add([], type::thing("ModelParams", $modelUID))
} RETURN uid;
------------------------------------------------------------

------------------------------------------------------------------------------------------------------------------------------

COMMIT TRANSACTION;