import React, { useState } from "react";
import "./quiz.css";

export interface QuizProps {
  statement: string;
  options: boolean[];
  answer: boolean;
  explanation: string;
}

export interface QuizFormProps {
  quizData: QuizProps[];
}

export const Quiz: React.FC<QuizFormProps> = ({ quizData }) => {
  const [currentIndex, setCurrentIndex] = useState(0);
  const [selectedAnswers, setselectedAnswers] = useState<boolean[]>([]);
  const [score, setScore] = useState(0);
  const [isQuizCompleted, setIsQuizCompleted] = useState(false);
  const options = [true, false];

  const handleOptionClick = (option: boolean) => {
    if (currentIndex < quizData.length) {
      const isCorrect = option === quizData[currentIndex].answer;
      setselectedAnswers([...selectedAnswers, option]);
      setScore(isCorrect ? score + 1 : score);
    }
    console.log(`currentIndex: ${currentIndex}`);
    console.log(`quizData.length: ${quizData.length}`);
    if (currentIndex + 1 === quizData.length) {
      setIsQuizCompleted(true);
    }
  };

  const handleNextQuestion = () => {
    if (currentIndex < quizData.length - 1) {
      setCurrentIndex(currentIndex + 1);
    }
  };

  const handleScoreResponse = (totalQuestions: number, score: number) => {
    const percentage = (score / totalQuestions) * 100;
    if (percentage >= 80) {
      return "Excellent vous avez plus de 80% de bonnes réponses";
    } else if (percentage >= 60) {
      return "Bien vous avez plus de 60% de bonnes réponses";
    } else if (percentage >= 40) {
      return "Moyen vous avez plus de 40% de bonnes réponses";
    } else {
      return "Il serait peut-être temps de réviser vos classiques";
    }
  };

  return (
    <div className="quiz-container">
      {!isQuizCompleted && (
        <>
          <div className="quiz-card">
            <h3>{quizData[currentIndex].statement}</h3>
            <ul>
              {options.map((option: boolean, index: React.Key) => (
                <li
                  key={index}
                  className={
                    currentIndex < selectedAnswers.length
                      ? option === quizData[currentIndex].answer
                        ? "correct"
                        : selectedAnswers[currentIndex] === option
                        ? "incorrect"
                        : ""
                      : selectedAnswers[currentIndex] === option
                      ? "selected"
                      : ""
                  }
                  onClick={() => handleOptionClick(option)}
                >
                  {option ? "Vrai" : "Faux"}
                </li>
              ))}
            </ul>
            {currentIndex < selectedAnswers.length && (
              <p>Explication : {quizData[currentIndex].explanation}</p>
            )}
          </div>

          <button className="next-button-quiz" onClick={handleNextQuestion}>
            Suivant
          </button>
        </>
      )}

      {isQuizCompleted && (
        <div className="quiz-summary-container">
          <div className="quiz-summary">
            <h2>Quiz terminé</h2>
            <p>{handleScoreResponse(quizData.length, score)}</p>
            <h2>Liste des résultats</h2>
            <div className="result-list-container">
              <ul>
                {quizData.map((quiz, index) => (
                  <li
                    style={{ cursor: "text" }}
                    key={index}
                    className={
                      quiz.answer === selectedAnswers[index]
                        ? "correct"
                        : "incorrect"
                    }
                  >
                    {index + 1} : {quiz.statement}
                  </li>
                ))}
              </ul>
            </div>
          </div>
        </div>
      )}
    </div>
  );
};
