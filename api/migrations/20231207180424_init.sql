-- Create the questions table
CREATE TABLE "questions" (
  "id" VARCHAR(36) NOT NULL PRIMARY KEY,
  "statement" VARCHAR(255) NOT NULL,
  "answer" BOOLEAN NOT NULL,
  "explanation" VARCHAR(500) NOT NULL,
  "attempts" INTEGER NOT NULL,
  "correct_answers" INTEGER NOT NULL
);

CREATE TABLE "scoreboard" (
  "id" VARCHAR(36) NOT NULL PRIMARY KEY,
  "name" VARCHAR(50) NOT NULL,
  "longest_streak" INTEGER NOT NULL
);
