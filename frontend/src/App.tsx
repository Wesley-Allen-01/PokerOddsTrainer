import { useState, useEffect} from 'react';
import init, { draw_starting_hand, draw_flop, draw_card, evaluate_hand, card_to_str} from "./wasm/odds_engine";

function App() {
  const [hand, setHand] = useState<number[] | null>(null);
  const [flop, setFlop] = useState<number[] | null>(null);
  const [evaluation, setEvaluation] = useState<number | null>(null);
  
  useEffect(() => {
    init().then(() => {
      console.log("WASM initialized");
    });
  }, []);

  const handleDraw = () => {
    const drawn = draw_starting_hand();
    setHand(Array.from(drawn));
  };

  const handleFlop = () => {
    const flop = draw_flop();
    setFlop(Array.from(flop));
  };

  const handleEval = () => {
    if (hand && flop) {
    const input = new Uint32Array([...hand, ...flop]);
    const ev = evaluate_hand(input)
    setEvaluation(ev)
    }
  };
  

  return (
    <div className="App">
      <h1>Poker Odds Trainer</h1>
      <button onClick={handleDraw}>Draw Cards</button>
      {hand && (
        <p>
          You drew: {card_to_str(hand[0])} and {card_to_str(hand[1])}
        </p>
      )}
      <button onClick={handleFlop}>Draw Flop</button>
      {flop && (
        <p>
          You drew: {card_to_str(flop[0])}, {card_to_str(flop[1])}, {card_to_str(flop[2])}
        </p>
      )}
      {flop && hand && (
        <button onClick={handleEval}> Evaluate </button>
      )}
      {evaluation && (
        <p> Your hand is equivalent to {evaluation}</p>
      )}
    </div>
  )
}

export default App;
