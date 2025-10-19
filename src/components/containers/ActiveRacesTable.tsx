import { AllVenuesResponse } from "../../types";

interface ActiveRacesTableProps {
  allVenues: AllVenuesResponse | null;
  onRaceSelect: (placeId: number, placeName: string, raceNumber: number) => void;
}

export function ActiveRacesTable({ allVenues, onRaceSelect }: ActiveRacesTableProps) {
  if (!allVenues) return null;

  // 開催中と非開催の競艇場を分ける
  const activeVenues = allVenues.venues.filter(venue => venue.is_active);
  const inactiveVenues = allVenues.venues.filter(venue => !venue.is_active);

  return (
    <div className="active-races-table">
      <h2>競艇場一覧 ({allVenues.date})</h2>
      
      {/* 開催中の競艇場 */}
      {activeVenues.length > 0 && (
        <div className="active-venues-section">
          <h3 className="section-title">開催中の競艇場</h3>
          <div className="active-races-grid">
            {activeVenues.map((venue) => (
              <div key={venue.place_id} className="venue-card active">
                <h4 className="venue-name">{venue.place_name}</h4>
                <div className="races-buttons">
                  {venue.races.map((raceNumber) => (
                    <button
                      key={raceNumber}
                      className="race-button"
                      onClick={() => onRaceSelect(venue.place_id, venue.place_name, raceNumber)}
                    >
                      {raceNumber}R
                    </button>
                  ))}
                </div>
              </div>
            ))}
          </div>
        </div>
      )}
      
      {/* 非開催の競艇場（まとめて表示） */}
      {inactiveVenues.length > 0 && (
        <div className="inactive-venues-section">
          <h3 className="section-title">開催なし</h3>
          <div className="inactive-venues-list">
            {inactiveVenues.map((venue) => (
              <span key={venue.place_id} className="inactive-venue-name">
                {venue.place_name}
              </span>
            ))}
          </div>
        </div>
      )}
    </div>
  );
}