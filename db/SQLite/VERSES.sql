SELECT b.ID AS BOOK,
       c.NUMBER AS CHAPTER,
       v.NUMBER AS VERSE,
       v.CONTENT AS TEXT
  FROM BOOK AS b
       JOIN
       CHAPTER AS c ON b.ID = c.BOOK_ID
       JOIN
       VERSE AS v ON v.CHAPTER_ID = c.ID
