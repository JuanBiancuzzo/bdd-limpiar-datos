CREATE TABLE IF NOT EXISTS NetflixAppVersion (
    versionId INTEGER PRIMARY KEY AUTOINCREMENT,
    versionNumber VARCHAR(20),
    buildNumber VARCHAR(10),
    buildCode VARCHAR(10)
);

CREATE TABLE IF NOT EXISTS NetflixReview (
    reviewId CHAR(36) PRIMARY KEY,
    userName VARCHAR(100),
    content TEXT,
    score INT,
    thumbsUpCount INT,
    createdAt TIMESTAMP,
    versionId INT,
    FOREIGN KEY (versionId) REFERENCES NetflixAppVersion(versionId)
);