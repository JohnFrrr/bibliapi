SELECT B.NAME,
       V.CHAPTER_ID,
       C.NUMBER AS CHAPTER_N,
       count(V.CHAPTER_ID) AS [count(V.CHAPTER_ID)],
       V.NUMBER AS VERSE_N,
       count(V.NUMBER) AS [count(V.NUMBER)]
  FROM VERSE AS V
       LEFT JOIN
       CHAPTER AS C ON C.ID = V.CHAPTER_ID
       LEFT JOIN
       BOOK AS B ON C.BOOK_ID = B.ID
 GROUP BY CHAPTER_ID,
          V.NUMBER
HAVING count(V.CHAPTER_ID) > 1 AND 
       count(V.NUMBER) > 1;
