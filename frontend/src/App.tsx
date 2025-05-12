import { useState, useEffect} from 'react';
import init, { draw_random_hand } from "./wasm/odds_engine";

function App() {
  const [hand, setHand] = useState<string[] | null>(null);
  
  useEffect(() => {
    init().then(() => {
      console.log("WASM initialized");
    });
  }, []);

  const handleDraw = () => {
    const drawn = draw_random_hand();
    setHand(drawn);
  };

  return (
    <div className="App">
      <h1>Poker Odds Trainer</h1>
      <button onClick={handleDraw}>Draw Cards</button>
      {hand && (
        <p>
          You drew: {hand[0]} and {hand[1]}
        </p>
      )}
    </div>
  )
}

export default App;
