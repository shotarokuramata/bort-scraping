import { RaceData } from "../../types";
import { 
  PlayerInfoSection, 
  BasicStatsSection, 
  PerformanceSection, 
  STSection, 
  WinningHandSection 
} from "../sections";

interface RaceDataContainerProps {
  raceData: RaceData | null;
}

export function RaceDataContainer({ raceData }: RaceDataContainerProps) {
  if (!raceData) return null;

  return (
    <div className="race-data">
      <PlayerInfoSection playerInfo={raceData.player_basic_info} />
      
      <BasicStatsSection
        escapeLastHalfYear={raceData.escape_last_half_year}
        escapeLastYear={raceData.escape_last_year}
        allowEscapeLastHalfYear={raceData.allow_escape_last_half_year}
        allowEscapeLastYear={raceData.allow_escape_last_year}
        pierceLastHalfYear={raceData.pierce_last_half_year}
        pierceLastYear={raceData.pierce_last_year}
        overtakeLastHalfYear={raceData.overtake_last_half_year}
        overtakeLastYear={raceData.overtake_last_year}
        firstPlaceInLastTenRace={raceData.first_place_in_last_ten_race}
      />
      
      <PerformanceSection performance={raceData.detailed_performance} />
      
      <STSection stData={raceData.st_data} />
      
      <WinningHandSection winningHand={raceData.winning_hand} />
    </div>
  );
}