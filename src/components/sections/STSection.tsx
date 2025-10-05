import { STRelatedData } from "../../types";

interface STSectionProps {
  stData: STRelatedData;
}

export function STSection({ stData }: STSectionProps) {
  return (
    <div className="st-section">
      <h2>ST関連データ</h2>
      
      <div className="st-grid">
        {/* 平均ST */}
        <div className="st-card">
          <h3>平均ST</h3>
          <div className="st-data">
            {stData.average_st.this_period !== undefined && (
              <div className="st-item">
                <span>今期:</span> {stData.average_st.this_period.toFixed(2)}
              </div>
            )}
            {stData.average_st.last_6_months !== undefined && (
              <div className="st-item">
                <span>直近6ヶ月:</span> {stData.average_st.last_6_months.toFixed(2)}
              </div>
            )}
            {stData.average_st.last_3_months !== undefined && (
              <div className="st-item">
                <span>直近3ヶ月:</span> {stData.average_st.last_3_months.toFixed(2)}
              </div>
            )}
            {stData.average_st.local_venue !== undefined && (
              <div className="st-item">
                <span>当地:</span> {stData.average_st.local_venue.toFixed(2)}
              </div>
            )}
            {stData.average_st.general_races !== undefined && (
              <div className="st-item">
                <span>一般戦:</span> {stData.average_st.general_races.toFixed(2)}
              </div>
            )}
            {stData.average_st.night_races !== undefined && (
              <div className="st-item">
                <span>ナイター:</span> {stData.average_st.night_races.toFixed(2)}
              </div>
            )}
          </div>
        </div>

        {/* ST順位 */}
        <div className="st-card">
          <h3>ST順位</h3>
          <div className="st-data">
            {stData.st_ranking.this_period !== undefined && (
              <div className="st-item">
                <span>今期:</span> {stData.st_ranking.this_period.toFixed(2)}
              </div>
            )}
            {stData.st_ranking.last_6_months !== undefined && (
              <div className="st-item">
                <span>直近6ヶ月:</span> {stData.st_ranking.last_6_months.toFixed(2)}
              </div>
            )}
            {stData.st_ranking.last_3_months !== undefined && (
              <div className="st-item">
                <span>直近3ヶ月:</span> {stData.st_ranking.last_3_months.toFixed(2)}
              </div>
            )}
            {stData.st_ranking.local_venue !== undefined && (
              <div className="st-item">
                <span>当地:</span> {stData.st_ranking.local_venue.toFixed(2)}
              </div>
            )}
            {stData.st_ranking.general_races !== undefined && (
              <div className="st-item">
                <span>一般戦:</span> {stData.st_ranking.general_races.toFixed(2)}
              </div>
            )}
          </div>
        </div>

        {/* ST考察 */}
        <div className="st-card">
          <h3>ST考察</h3>
          <div className="st-data">
            {stData.st_analysis.stability_rate !== undefined && (
              <div className="st-item">
                <span>安定率:</span> {(stData.st_analysis.stability_rate * 100).toFixed(1)}%
              </div>
            )}
            {stData.st_analysis.break_out_rate !== undefined && (
              <div className="st-item">
                <span>抜出率:</span> {(stData.st_analysis.break_out_rate * 100).toFixed(1)}%
              </div>
            )}
            {stData.st_analysis.late_start_rate !== undefined && (
              <div className="st-item">
                <span>出遅率:</span> {(stData.st_analysis.late_start_rate * 100).toFixed(1)}%
              </div>
            )}
          </div>
        </div>
      </div>
    </div>
  );
}