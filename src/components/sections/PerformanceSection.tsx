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
      </div>
    </div>
  );
}