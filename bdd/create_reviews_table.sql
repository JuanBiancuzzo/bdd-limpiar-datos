CREATE TABLE IF NOT EXISTS NetflixAppVersion (
    versionID INTEGER PRIMARY KEY AUTOINCREMENT,
    version TEXT,
    buildNumber VARCHAR(10),
    buildCode VARCHAR(10)
);

CREATE TABLE IF NOT EXISTS NetflixReview (
    reviewId CHAR(36) PRIMARY KEY UNIQUE,
    userID VARCHAR(100),
    content TEXT,
    score INT,
    thumbsUpCount INT,
    createdAt TIMESTAMP,
    versionId INT,
    FOREIGN KEY (versionId) REFERENCES NetflixAppVersion(versionId)
    FOREIGN KEY (userID) REFERENCES NetflixUser(userID)
);

CREATE TABLE IF NOT EXISTS NetflixUser (
    userID INTEGER PRIMARY KEY AUTOINCREMENT,
    userName VARCHAR(100)
);
