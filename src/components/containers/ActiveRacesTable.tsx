import { ActiveRace } from "../../types";

interface ActiveRacesTableProps {
  activeRaces: ActiveRace | null;
  onRaceSelect: (placeId: number, placeName: string, raceNumber: number) => void;
}

export function ActiveRacesTable({ activeRaces, onRaceSelect }: ActiveRacesTableProps) {
  if (!activeRaces) return null;

  return (
    <div className="active-races-table">
      <h2>開催レース場一覧 ({activeRaces.date})</h2>
      
      <div className="races-grid">
        {activeRaces.venues.map((venue) => (
          <div key={venue.place_id} className="venue-card">
            <h3 className="venue-name">{venue.place_name}</h3>
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
  );
}