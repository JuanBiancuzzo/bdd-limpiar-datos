GET_VERSION_ID_QUERY = """
SELECT versionId 
FROM NetflixAppVersion 
WHERE version = ? AND buildNumber = ? AND buildCode = ?
"""

INSERT_VERSION_QUERY = """
INSERT INTO NetflixAppVersion (version, buildNumber, buildCode) 
VALUES (?, ?, ?)
"""

GET_USER_ID_QUERY = """
SELECT userId 
FROM NetflixUser 
WHERE userName = ?
"""

INSERT_USER_QUERY = """
INSERT INTO NetflixUser (userName) 
VALUES (?)
"""

INSERT_REVIEW_QUERY = """
INSERT INTO NetflixReview (reviewID, userID, content, score, thumbsUpCount, createdAt, versionId) 
VALUES (?, ?, ?, ?, ?, ?, ?)
"""