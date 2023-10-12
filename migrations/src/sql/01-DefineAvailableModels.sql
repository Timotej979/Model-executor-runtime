BEGIN TRANSACTION;

----------------------------------------------------------------------------------------------------------
-- Define dynamic connTypeParams table
DEFINE TABLE connTypeParams SCHEMALESS;
-- Define uid and order and make them unique
DEFINE FIELD uid ON TABLE connTypeParams TYPE string ASSERT $value != NONE AND is::uuid($value);
DEFINE INDEX order ON TABLE connTypeParams COLUMNS uid, order UNIQUE; 
DEFINE FIELD createdAt ON TABLE connTypeParams TYPE datetime ASSERT $value != NONE AND $value != NULL;
-----------------------------------------------------------------------------------------------------------

-----------------------------------------------------------------------------------------------------------
-- Define dynamic modelParams table
DEFINE TABLE modelParams SCHEMALESS;
-- Define uid and order and make them unique
DEFINE FIELD uid ON TABLE modelParams TYPE string ASSERT $value != NONE AND is::uuid($value);
DEFINE INDEX order ON TABLE modelParams COLUMN uid, order UNIQUE;
DEFINE FIELD createdAt ON TABLE modelParams TYPE datetime ASSERT $value != NONE AND $value != NULL;
-----------------------------------------------------------------------------------------------------------

------------------------------------------------------------------------------------------------------------
-- Define static AvailableModels table
DEFINE TABLE AvailableModels SCHEMAFULL;

-- Define uid and order and make the unique
DEFINE FIELD uid ON TABLE AvailableModels TYPE string ASSERT $value != NONE AND is::uuid($value);
DEFINE INDEX order ON TABLE AvailableModels COLUMNS uid, order UNIQUE;

-- Define name and type
DEFINE FIELD name ON TABLE AvailableModels TYPE string ASSERT $value != NONE AND $value != NULL;
DEFINE FIELD connType ON TABLE AvailableModels TYPE string ASSERT $value != NONE AND $value != NULL;

-- Define createdAt
DEFINE FIELD createdAt ON TABLE AvailableModels TYPE datetime ASSERT $value != NONE AND $value != NULL;

-- Define dynamic connTypeParams table
DEFINE FIELD connTypeParams ON TABLE AvailableModels TYPE array;  
DEFINE FIELD connTypeParams.* ON TABLE AvailableModels TYPE record(connTypeParams);

-- Define dynamic modelParams table
DEFINE FIELD modelParams ON TABLE AvailableModels TYPE array;
DEFIN FIELD modelParams.* ON TABLE AvailableModels TYPE record(modelParams);
--------------------------------------------------------------------------------------------------------------

COMMIT TRANSACTION;