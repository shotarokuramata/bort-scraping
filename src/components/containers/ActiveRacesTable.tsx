import { AllVenuesResponse } from "../../types";

interface ActiveRacesTableProps {
  allVenues: AllVenuesResponse | null;
  onRaceSelect: (placeId: number, placeName: string, raceNumber: number) => void;
}

export function ActiveRacesTable({ allVenues, onRaceSelect }: ActiveRacesTableProps) {
  if (!allVenues) return null;

  return (
    <div className="active-races-table">
      <h2>競艇場一覧 ({allVenues.date})</h2>
      
      <div className="races-grid">
        {allVenues.venues.map((venue) => (
          <div key={venue.place_id} className={`venue-card ${venue.is_active ? 'active' : 'inactive'}`}>
            <h3 className="venue-name">{venue.place_name}</h3>
            <div className="races-buttons">
              {venue.is_active ? (
                venue.races.map((raceNumber) => (
                  <button
                    key={raceNumber}
                    className="race-button"
                    onClick={() => onRaceSelect(venue.place_id, venue.place_name, raceNumber)}
                  >
                    {raceNumber}R
                  </button>
                ))
              ) : (
                <div className="no-races">開催なし</div>
              )}
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}