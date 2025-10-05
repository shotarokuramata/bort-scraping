import { PlayerBasicInfo } from "../../types";

interface PlayerInfoSectionProps {
  playerInfo: PlayerBasicInfo;
}

export function PlayerInfoSection({ playerInfo }: PlayerInfoSectionProps) {
  return (
    <div className="player-info-section">
      <h2>選手基本情報（1号艇）</h2>
      <div className="player-basic-info">
        <div className="info-grid">
          <div className="info-item">
            <span className="info-label">選手名:</span>
            <span className="info-value">{playerInfo.name}</span>
          </div>
          <div className="info-item">
            <span className="info-label">登録番号:</span>
            <span className="info-value">{playerInfo.registration_number}</span>
          </div>
          <div className="info-item">
            <span className="info-label">級別:</span>
            <span className="info-value">{playerInfo.class_level}</span>
          </div>
          <div className="info-item">
            <span className="info-label">期別:</span>
            <span className="info-value">{playerInfo.period}</span>
          </div>
          <div className="info-item">
            <span className="info-label">支部:</span>
            <span className="info-value">{playerInfo.support_group}</span>
          </div>
          <div className="info-item">
            <span className="info-label">性別:</span>
            <span className="info-value">{playerInfo.gender}</span>
          </div>
        </div>
      </div>
    </div>
  );
}