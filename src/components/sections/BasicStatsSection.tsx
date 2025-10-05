interface BasicStatsSectionProps {
  escapeLastHalfYear: number;
  escapeLastYear: number;
  allowEscapeLastHalfYear: number;
  allowEscapeLastYear: number;
  pierceLastHalfYear: number;
  pierceLastYear: number;
  overtakeLastHalfYear: number;
  overtakeLastYear: number;
  firstPlaceInLastTenRace: number;
}

export function BasicStatsSection({
  escapeLastHalfYear,
  escapeLastYear,
  allowEscapeLastHalfYear,
  allowEscapeLastYear,
  pierceLastHalfYear,
  pierceLastYear,
  overtakeLastHalfYear,
  overtakeLastYear,
  firstPlaceInLastTenRace,
}: BasicStatsSectionProps) {
  return (
    <div className="basic-stats-section">
      <h2>基本統計データ</h2>
      <table className="data-table">
        <thead>
          <tr>
            <th>項目</th>
            <th>半年間</th>
            <th>1年間</th>
          </tr>
        </thead>
        <tbody>
          <tr>
            <td>逃げ率</td>
            <td>{(escapeLastHalfYear * 100).toFixed(1)}%</td>
            <td>{(escapeLastYear * 100).toFixed(1)}%</td>
          </tr>
          <tr>
            <td>逃がし率</td>
            <td>{(allowEscapeLastHalfYear * 100).toFixed(1)}%</td>
            <td>{(allowEscapeLastYear * 100).toFixed(1)}%</td>
          </tr>
          <tr>
            <td>差され率</td>
            <td>{(pierceLastHalfYear * 100).toFixed(1)}%</td>
            <td>{(pierceLastYear * 100).toFixed(1)}%</td>
          </tr>
          <tr>
            <td>捲られ率</td>
            <td>{(overtakeLastHalfYear * 100).toFixed(1)}%</td>
            <td>{(overtakeLastYear * 100).toFixed(1)}%</td>
          </tr>
        </tbody>
      </table>
      
      <div className="additional-info">
        <h3>直近10レースの成績</h3>
        <div className="stat-item">
          <span className="stat-label">1着回数:</span>
          <span className="stat-value">{firstPlaceInLastTenRace}回</span>
        </div>
      </div>
    </div>
  );
}