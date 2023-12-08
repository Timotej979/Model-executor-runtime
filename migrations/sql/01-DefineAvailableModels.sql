BEGIN TRANSACTION;

----------------------------------------------------------------------------------------------------------
-- Define dynamic connTypeParams table
DEFINE TABLE ConnTypeParams SCHEMALESS;
-- Define uid and order and make them unique
DEFINE FIELD uid ON TABLE ConnTypeParams TYPE string ASSERT $value != NONE AND $value != NULL;
DEFINE INDEX order ON TABLE ConnTypeParams COLUMNS uid UNIQUE; 
DEFINE FIELD createdAt ON TABLE ConnTypeParams TYPE datetime ASSERT $value != NONE AND $value != NULL;
DEFINE FIELD lastUpdated ON TABLE ConnTypeParams TYPE datetime ASSERT $value != NONE AND $value != NULL;
-----------------------------------------------------------------------------------------------------------

-----------------------------------------------------------------------------------------------------------
-- Define dynamic modelParams table
DEFINE TABLE ModelParams SCHEMALESS;
-- Define uid and order and make them unique
DEFINE FIELD uid ON TABLE ModelParams TYPE string ASSERT $value != NONE AND $value != NULL;
DEFINE INDEX order ON TABLE ModelParams COLUMNS uid UNIQUE;
DEFINE FIELD createdAt ON TABLE ModelParams TYPE datetime ASSERT $value != NONE AND $value != NULL;
DEFINE FIELD lastUpdated ON TABLE ModelParams TYPE datetime ASSERT $value != NONE AND $value != NULL;
-----------------------------------------------------------------------------------------------------------

------------------------------------------------------------------------------------------------------------
-- Define static AvailableModels table
DEFINE TABLE AvailableModels SCHEMAFULL;

-- Define uid and order and make the unique
DEFINE FIELD uid ON TABLE AvailableModels TYPE string ASSERT $value != NONE AND $value != NULL;
DEFINE INDEX order ON TABLE AvailableModels COLUMNS uid UNIQUE;

-- Define name and type
DEFINE FIELD name ON TABLE AvailableModels TYPE string ASSERT $value != NONE AND $value != NULL AND $value != NULL;
DEFINE FIELD connType ON TABLE AvailableModels TYPE string ASSERT $value != NONE AND $value != NULL;

-- Define createdAt
DEFINE FIELD createdAt ON TABLE AvailableModels TYPE datetime ASSERT $value != NONE AND $value != NULL;
DEFINE FIELD lastUpdated ON TABLE AvailableModels TYPE datetime ASSERT $value != NONE AND $value != NULL;

-- Define dynamic connTypeParams table
DEFINE FIELD connTypeParams ON TABLE AvailableModels TYPE array;  
DEFINE FIELD connTypeParams.* ON TABLE AvailableModels TYPE record(ConnTypeParams);

-- Define dynamic modelParams table
DEFINE FIELD modelParams ON TABLE AvailableModels TYPE array;
DEFINE FIELD modelParams.* ON TABLE AvailableModels TYPE record(ModelParams);
--------------------------------------------------------------------------------------------------------------

COMMIT TRANSACTION;