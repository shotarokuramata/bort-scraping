interface OddsItemProps {
  boatNumber: number;
  odds: number;
  rangeText?: string;
}

export function OddsItem({ boatNumber, odds, rangeText }: OddsItemProps) {
  return (
    <div className="win-place-odds-item">
      <span className="boat-number">{boatNumber}号艇</span>
      <span className="odds-value">{rangeText || `${odds.toFixed(1)}倍`}</span>
    </div>
  );
}