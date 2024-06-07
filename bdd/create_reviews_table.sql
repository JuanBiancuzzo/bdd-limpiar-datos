CREATE TABLE IF NOT EXISTS NetflixAppVersion (
    versionID INTEGER PRIMARY KEY AUTOINCREMENT,
    version TEXT,
    buildNumber VARCHAR(10),
    buildCode VARCHAR(10)
);

CREATE TABLE IF NOT EXISTS NetflixReview (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    reviewId CHAR(36),
    userID VARCHAR(100),
    content TEXT,
    score INT,
    thumbsUpCount INT,
    createdAt TIMESTAMP,
    versionId INT,
    FOREIGN KEY (versionId) REFERENCES NetflixAppVersion(versionId)
);

CREATE TABLE IF NOT EXISTS NetflixUser (
    userId INTEGER PRIMARY KEY AUTOINCREMENT,
    userName VARCHAR(100)
);