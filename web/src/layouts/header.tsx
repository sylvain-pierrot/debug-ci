import React, { useState, useEffect } from "react";
import "./header.css";

export interface HeaderProps {
  message: string;
}
enum TimeManagement {
  START_TIME = 1980,
  END_TIME = 2024,
  DURATION_IN_MINUTES = 1,
}
export const Header: React.FC<HeaderProps> = ({ message }) => {
  const [year, setYear] = useState(TimeManagement.START_TIME);

  useEffect(() => {
    const endTime = TimeManagement.END_TIME;
    const startTime = year;
    const totalTimeInYears = endTime - startTime;
    const durationInMinutes = 1;
    const totalIntervals = totalTimeInYears * 12;

    let currentInterval = 0;

    const interval = setInterval(() => {
      const currentYear =
        startTime +
        Math.floor((currentInterval / totalIntervals) * totalTimeInYears);
      setYear(currentYear);

      currentInterval += 1;

      if (currentInterval >= totalIntervals) {
        clearInterval(interval);
      }
    }, (durationInMinutes * 60 * 1000) / totalIntervals);

    return () => clearInterval(interval);
  }, []);
  return (
    <>
      <header className="app-header">
        <h1 className="logo">{message}</h1>
        <div className="right-part-header">
          <input className="tgl tgl-skewed" id="cb3" type="checkbox" />
          <label
            onClick={() => console.log("Eco mode activé")}
            className="tgl-btn"
            data-tg-off="OFF"
            data-tg-on="ON"
            htmlFor="cb3"
          ></label>

          <button onClick={() => console.log("theme changed")}>Thème</button>
          <p className="year">{year}</p>
        </div>
      </header>
    </>
  );
};
