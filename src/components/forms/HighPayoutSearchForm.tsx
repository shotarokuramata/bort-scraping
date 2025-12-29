import React, { useState } from "react";
import { PayoutType } from "../../types/OpenApiData";

interface HighPayoutSearchFormProps {
  onSearch: (minPayout: number, payoutType: PayoutType, limit: number) => void;
  isLoading: boolean;
}

export const HighPayoutSearchForm: React.FC<HighPayoutSearchFormProps> = ({
  onSearch,
  isLoading,
}) => {
  const [minPayout, setMinPayout] = useState<number>(100000);
  const [payoutType, setPayoutType] = useState<PayoutType>("trifecta");
  const [limit, setLimit] = useState<number>(20);

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    onSearch(minPayout, payoutType, limit);
  };

  return (
    <form onSubmit={handleSubmit} className="search-form">
      <div className="form-row">
        <div className="form-group">
          <label htmlFor="payoutType">配当種類</label>
          <select
            id="payoutType"
            value={payoutType}
            onChange={(e) => setPayoutType(e.target.value as PayoutType)}
            disabled={isLoading}
          >
            <option value="trifecta">3連単</option>
            <option value="exacta">2連単</option>
            <option value="win">単勝</option>
            <option value="place">複勝</option>
          </select>
        </div>

        <div className="form-group">
          <label htmlFor="minPayout">最低配当額（円）</label>
          <input
            id="minPayout"
            type="number"
            value={minPayout}
            onChange={(e) => setMinPayout(Number(e.target.value))}
            min={0}
            step={1000}
            disabled={isLoading}
          />
        </div>

        <div className="form-group">
          <label htmlFor="limit">取得件数</label>
          <input
            id="limit"
            type="number"
            value={limit}
            onChange={(e) => setLimit(Number(e.target.value))}
            min={1}
            max={100}
            disabled={isLoading}
          />
        </div>

        <button type="submit" disabled={isLoading} className="search-button">
          {isLoading ? "検索中..." : "検索"}
        </button>
      </div>
    </form>
  );
};
