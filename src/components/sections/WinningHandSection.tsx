import { WinningHandData } from "../../types";

interface WinningHandSectionProps {
  winningHand: WinningHandData;
}

export function WinningHandSection({ winningHand }: WinningHandSectionProps) {
  return (
    <div className="winning-hand-section">
      <h2>決まり手データ（直近6ヶ月）</h2>
      
      <div className="winning-hand-grid">
        {/* 1号艇の決まり手 */}
        <div className="winning-hand-card">
          <h3>1号艇の決まり手</h3>
          <div className="winning-hand-data">
            {winningHand.escape_rate_6months !== undefined && (
              <div className="winning-hand-item">
                <span>逃げ率:</span> {(winningHand.escape_rate_6months * 100).toFixed(1)}%
              </div>
            )}
            {winningHand.pierced_rate_6months !== undefined && (
              <div className="winning-hand-item">
                <span>差され率:</span> {(winningHand.pierced_rate_6months * 100).toFixed(1)}%
              </div>
            )}
            {winningHand.overtake_rate_6months !== undefined && (
              <div className="winning-hand-item">
                <span>捲られ率:</span> {(winningHand.overtake_rate_6months * 100).toFixed(1)}%
              </div>
            )}
          </div>
        </div>

        {/* 2号艇以降の決まり手 */}
        <div className="winning-hand-card">
          <h3>2号艇以降の決まり手</h3>
          <div className="winning-hand-data">
            {winningHand.pierce_rate_6months !== undefined && (
              <div className="winning-hand-item">
                <span>2号艇差し率:</span> {(winningHand.pierce_rate_6months * 100).toFixed(1)}%
              </div>
            )}
            {winningHand.let_escape_rate_6months !== undefined && (
              <div className="winning-hand-item">
                <span>逃し率:</span> {(winningHand.let_escape_rate_6months * 100).toFixed(1)}%
              </div>
            )}
          </div>
        </div>
      </div>
    </div>
  );
}