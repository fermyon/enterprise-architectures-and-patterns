CREATE TABLE IF NOT EXISTS Items (
   Id varchar(36) PRIMARY KEY,
   Name TEXT NOT NULL,
   Active BOOLEAN NOT NULL
);

INSERT INTO Items(Id, Name, Active)
SELECT '9ccc555f-e4f8-446c-83cb-ee1542862450', 'Lane Assistant', TRUE
WHERE
NOT EXISTS (
SELECT Id FROM Items WHERE Id = '9ccc555f-e4f8-446c-83cb-ee1542862450'
);

INSERT INTO Items(Id, Name, Active)
SELECT '35499593-ff71-4b13-8bd5-07ddf47bee6b', 'Sentry Mode', TRUE
WHERE
NOT EXISTS (
SELECT Id FROM Items WHERE Id = '35499593-ff71-4b13-8bd5-07ddf47bee6b'
);

INSERT INTO Items(Id, Name, Active)
SELECT 'eb74da9d-3bb5-4f4f-825a-9bb628f5691b', 'Autopilot', FALSE
WHERE
NOT EXISTS (
SELECT Id FROM Items WHERE Id = 'eb74da9d-3bb5-4f4f-825a-9bb628f5691b'
);
