import React from 'react';
import Spline from "@splinetool/react-spline";
import { useRef } from 'react';
    
export const EarthScene: React.FC = () => {
  const spline = useRef();

    function onLoad(splineApp) {
        // save the app in a ref for later use
        spline.current = splineApp; 
      }

      function triggerAnimation() {
        if (spline.current) {
          document.dispatchEvent(new KeyboardEvent("keydown", {
            key: "s",
            keyCode: 69,
            code: "KeyE",  
            which: 69,
            shiftKey: false, 
            ctrlKey: false,   
            metaKey: false 
        }));
        }
      }

      document.onkeydown = function (e) 
        {
          console.log("here")
          e.preventDefault();
          return true;
        }


  return (
    <div style={{height: "100%"}}>
              <button type="button" onClick={triggerAnimation}>
        Trigger Spline Animation
      </button>
        <Spline scene="https://prod.spline.design/VNrzxGNkm-IEv1K3/scene.splinecode" onLoad={onLoad} style={{transform: "scale(1)", zIndex: -10}}/>

    </div>
    
  );
};
