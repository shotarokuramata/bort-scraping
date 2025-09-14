import { DetailedPerformanceData } from "../../types";
import { PerformanceItem } from "../parts";

interface PerformanceSectionProps {
  performance: DetailedPerformanceData;
}

export function PerformanceSection({ performance }: PerformanceSectionProps) {
  return (
    <div className="performance-section">
      <h2>詳細成績データ</h2>
      
      <div className="performance-grid">
        {/* 1着率 */}
        <div className="performance-card">
          <h3>1着率</h3>
          <div className="performance-data">
            <PerformanceItem label="今期" value={performance.first_place_rate.this_period} />
            <PerformanceItem label="直近6ヶ月" value={performance.first_place_rate.last_6_months} />
            <PerformanceItem label="直近3ヶ月" value={performance.first_place_rate.last_3_months} />
            <PerformanceItem label="直近1ヶ月" value={performance.first_place_rate.last_1_month} />
            <PerformanceItem label="当地" value={performance.first_place_rate.local_venue} />
            <PerformanceItem label="一般戦" value={performance.first_place_rate.general_races} />
          </div>
        </div>

        {/* 2連対率 */}
        <div className="performance-card">
          <h3>2連対率</h3>
          <div className="performance-data">
            {performance.top_2_rate.this_period !== undefined && (
              <div className="perf-item">
                <span>今期:</span> {(performance.top_2_rate.this_period * 100).toFixed(1)}%
              </div>
            )}
            {performance.top_2_rate.last_6_months !== undefined && (
              <div className="perf-item">
                <span>直近6ヶ月:</span> {(performance.top_2_rate.last_6_months * 100).toFixed(1)}%
              </div>
            )}
            {performance.top_2_rate.last_3_months !== undefined && (
              <div className="perf-item">
                <span>直近3ヶ月:</span> {(performance.top_2_rate.last_3_months * 100).toFixed(1)}%
              </div>
            )}
            {performance.top_2_rate.last_1_month !== undefined && (
              <div className="perf-item">
                <span>直近1ヶ月:</span> {(performance.top_2_rate.last_1_month * 100).toFixed(1)}%
              </div>
            )}
            {performance.top_2_rate.local_venue !== undefined && (
              <div className="perf-item">
                <span>当地:</span> {(performance.top_2_rate.local_venue * 100).toFixed(1)}%
              </div>
            )}
            {performance.top_2_rate.general_races !== undefined && (
              <div className="perf-item">
                <span>一般戦:</span> {(performance.top_2_rate.general_races * 100).toFixed(1)}%
              </div>
            )}
          </div>
        </div>

        {/* 3連対率 */}
        <div className="performance-card">
          <h3>3連対率</h3>
          <div className="performance-data">
            {performance.top_3_rate.this_period !== undefined && (
              <div className="perf-item">
                <span>今期:</span> {(performance.top_3_rate.this_period * 100).toFixed(1)}%
              </div>
            )}
            {performance.top_3_rate.last_6_months !== undefined && (
              <div className="perf-item">
                <span>直近6ヶ月:</span> {(performance.top_3_rate.last_6_months * 100).toFixed(1)}%
              </div>
            )}
            {performance.top_3_rate.last_3_months !== undefined && (
              <div className="perf-item">
                <span>直近3ヶ月:</span> {(performance.top_3_rate.last_3_months * 100).toFixed(1)}%
              </div>
            )}
            {performance.top_3_rate.last_1_month !== undefined && (
              <div className="perf-item">
                <span>直近1ヶ月:</span> {(performance.top_3_rate.last_1_month * 100).toFixed(1)}%
              </div>
            )}
            {performance.top_3_rate.local_venue !== undefined && (
              <div className="perf-item">
                <span>当地:</span> {(performance.top_3_rate.local_venue * 100).toFixed(1)}%
              </div>
            )}
            {performance.top_3_rate.general_races !== undefined && (
              <div className="perf-item">
                <span>一般戦:</span> {(performance.top_3_rate.general_races * 100).toFixed(1)}%
              </div>
            )}
          </div>
        </div>
      </div>
    </div>
  );
}