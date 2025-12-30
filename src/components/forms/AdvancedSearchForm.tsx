import React, { useState } from "react";
import { SearchParams } from "../../types/AdvancedSearch";
import { kyoteiPlaces, racerClasses, raceGrades } from "../../information";

interface AdvancedSearchFormProps {
  onSearch: (params: SearchParams) => void;
  isLoading: boolean;
}

export const AdvancedSearchForm: React.FC<AdvancedSearchFormProps> = ({
  onSearch,
  isLoading,
}) => {
  const [showDetailed, setShowDetailed] = useState(false);
  const [formData, setFormData] = useState<SearchParams>({ limit: 50 });

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    onSearch(formData);
  };

  const handleInputChange = (
    field: keyof SearchParams,
    value: string | number | undefined
  ) => {
    setFormData((prev) => ({
      ...prev,
      [field]: value === "" ? undefined : value,
    }));
  };

  return (
    <form onSubmit={handleSubmit} className="search-form">
      {/* 基本検索セクション */}
      <div className="form-row">
        <div className="form-group">
          <label htmlFor="racer_name">選手名</label>
          <input
            id="racer_name"
            type="text"
            placeholder="例: 峰竜太"
            value={formData.racer_name || ""}
            onChange={(e) => handleInputChange("racer_name", e.target.value)}
            disabled={isLoading}
          />
        </div>

        <div className="form-group">
          <label htmlFor="racer_class">級別</label>
          <select
            id="racer_class"
            value={formData.racer_class || ""}
            onChange={(e) =>
              handleInputChange(
                "racer_class",
                e.target.value ? Number(e.target.value) : undefined
              )
            }
            disabled={isLoading}
          >
            <option value="">指定なし</option>
            {racerClasses.map((c) => (
              <option key={c.value} value={c.value}>
                {c.label}
              </option>
            ))}
          </select>
        </div>

        <div className="form-group">
          <label htmlFor="venue_code">会場</label>
          <select
            id="venue_code"
            value={formData.venue_code || ""}
            onChange={(e) => handleInputChange("venue_code", e.target.value)}
            disabled={isLoading}
          >
            <option value="">指定なし</option>
            {Object.entries(kyoteiPlaces).map(([code, name]) => (
              <option key={code} value={String(code).padStart(2, "0")}>
                {name}
              </option>
            ))}
          </select>
        </div>

        <div className="form-group">
          <label htmlFor="limit">取得件数</label>
          <input
            id="limit"
            type="number"
            value={formData.limit || 50}
            onChange={(e) => handleInputChange("limit", Number(e.target.value))}
            min={1}
            max={200}
            disabled={isLoading}
          />
        </div>
      </div>

      {/* 詳細検索切り替えボタン */}
      <div style={{ marginTop: "15px", marginBottom: "15px" }}>
        <button
          type="button"
          onClick={() => setShowDetailed(!showDetailed)}
          style={{
            padding: "8px 16px",
            backgroundColor: "#f5f5f5",
            border: "1px solid #ddd",
            cursor: "pointer",
            borderRadius: "4px",
          }}
        >
          {showDetailed ? "▲ 詳細条件を隠す" : "▼ 詳細条件を表示"}
        </button>
      </div>

      {/* 詳細検索セクション */}
      {showDetailed && (
        <div className="advanced-search-section">
          {/* 日付範囲 */}
          <h4>日付範囲</h4>
          <div className="form-row">
            <div className="form-group">
              <label htmlFor="date_from">開始日 (YYYYMMDD)</label>
              <input
                id="date_from"
                type="text"
                placeholder="20250101"
                value={formData.date_from || ""}
                onChange={(e) => handleInputChange("date_from", e.target.value)}
                disabled={isLoading}
                maxLength={8}
              />
            </div>
            <div className="form-group">
              <label htmlFor="date_to">終了日 (YYYYMMDD)</label>
              <input
                id="date_to"
                type="text"
                placeholder="20250131"
                value={formData.date_to || ""}
                onChange={(e) => handleInputChange("date_to", e.target.value)}
                disabled={isLoading}
                maxLength={8}
              />
            </div>
          </div>

          {/* レース条件 */}
          <h4>レース条件</h4>
          <div className="form-row">
            <div className="form-group">
              <label htmlFor="race_grade">レースグレード</label>
              <select
                id="race_grade"
                value={formData.race_grade || ""}
                onChange={(e) =>
                  handleInputChange(
                    "race_grade",
                    e.target.value ? Number(e.target.value) : undefined
                  )
                }
                disabled={isLoading}
              >
                <option value="">指定なし</option>
                {raceGrades.map((g) => (
                  <option key={g.value} value={g.value}>
                    {g.label}
                  </option>
                ))}
              </select>
            </div>
            <div className="form-group">
              <label htmlFor="race_number">レース番号</label>
              <input
                id="race_number"
                type="number"
                placeholder="1-12"
                value={formData.race_number || ""}
                onChange={(e) =>
                  handleInputChange(
                    "race_number",
                    e.target.value ? Number(e.target.value) : undefined
                  )
                }
                min={1}
                max={12}
                disabled={isLoading}
              />
            </div>
          </div>

          {/* 配当条件 */}
          <h4>配当条件</h4>
          <div className="form-row">
            <div className="form-group">
              <label htmlFor="min_trifecta_payout">3連単最低配当（円）</label>
              <input
                id="min_trifecta_payout"
                type="number"
                placeholder="100000"
                value={formData.min_trifecta_payout || ""}
                onChange={(e) =>
                  handleInputChange(
                    "min_trifecta_payout",
                    e.target.value ? Number(e.target.value) : undefined
                  )
                }
                min={0}
                step={10000}
                disabled={isLoading}
              />
            </div>
            <div className="form-group">
              <label htmlFor="max_trifecta_payout">3連単最高配当（円）</label>
              <input
                id="max_trifecta_payout"
                type="number"
                placeholder="1000000"
                value={formData.max_trifecta_payout || ""}
                onChange={(e) =>
                  handleInputChange(
                    "max_trifecta_payout",
                    e.target.value ? Number(e.target.value) : undefined
                  )
                }
                min={0}
                step={10000}
                disabled={isLoading}
              />
            </div>
            <div className="form-group">
              <label htmlFor="min_win_payout">単勝最低配当（円）</label>
              <input
                id="min_win_payout"
                type="number"
                placeholder="1000"
                value={formData.min_win_payout || ""}
                onChange={(e) =>
                  handleInputChange(
                    "min_win_payout",
                    e.target.value ? Number(e.target.value) : undefined
                  )
                }
                min={0}
                step={100}
                disabled={isLoading}
              />
            </div>
          </div>

          {/* 気象条件 */}
          <h4>気象条件</h4>
          <div className="form-row">
            <div className="form-group">
              <label htmlFor="min_wind">最低風速 (m)</label>
              <input
                id="min_wind"
                type="number"
                placeholder="0"
                value={formData.min_wind || ""}
                onChange={(e) =>
                  handleInputChange(
                    "min_wind",
                    e.target.value ? Number(e.target.value) : undefined
                  )
                }
                min={0}
                step={0.1}
                disabled={isLoading}
              />
            </div>
            <div className="form-group">
              <label htmlFor="max_wind">最高風速 (m)</label>
              <input
                id="max_wind"
                type="number"
                placeholder="10"
                value={formData.max_wind || ""}
                onChange={(e) =>
                  handleInputChange(
                    "max_wind",
                    e.target.value ? Number(e.target.value) : undefined
                  )
                }
                min={0}
                step={0.1}
                disabled={isLoading}
              />
            </div>
            <div className="form-group">
              <label htmlFor="min_wave">最低波高 (cm)</label>
              <input
                id="min_wave"
                type="number"
                placeholder="0"
                value={formData.min_wave || ""}
                onChange={(e) =>
                  handleInputChange(
                    "min_wave",
                    e.target.value ? Number(e.target.value) : undefined
                  )
                }
                min={0}
                step={1}
                disabled={isLoading}
              />
            </div>
            <div className="form-group">
              <label htmlFor="max_wave">最高波高 (cm)</label>
              <input
                id="max_wave"
                type="number"
                placeholder="50"
                value={formData.max_wave || ""}
                onChange={(e) =>
                  handleInputChange(
                    "max_wave",
                    e.target.value ? Number(e.target.value) : undefined
                  )
                }
                min={0}
                step={1}
                disabled={isLoading}
              />
            </div>
            <div className="form-group">
              <label htmlFor="min_temperature">最低気温 (°C)</label>
              <input
                id="min_temperature"
                type="number"
                placeholder="0"
                value={formData.min_temperature || ""}
                onChange={(e) =>
                  handleInputChange(
                    "min_temperature",
                    e.target.value ? Number(e.target.value) : undefined
                  )
                }
                step={0.1}
                disabled={isLoading}
              />
            </div>
            <div className="form-group">
              <label htmlFor="max_temperature">最高気温 (°C)</label>
              <input
                id="max_temperature"
                type="number"
                placeholder="40"
                value={formData.max_temperature || ""}
                onChange={(e) =>
                  handleInputChange(
                    "max_temperature",
                    e.target.value ? Number(e.target.value) : undefined
                  )
                }
                step={0.1}
                disabled={isLoading}
              />
            </div>
          </div>

          {/* その他条件 */}
          <h4>その他</h4>
          <div className="form-row">
            <div className="form-group">
              <label htmlFor="winner_boat_number">勝者艇番</label>
              <input
                id="winner_boat_number"
                type="number"
                placeholder="1-6"
                value={formData.winner_boat_number || ""}
                onChange={(e) =>
                  handleInputChange(
                    "winner_boat_number",
                    e.target.value ? Number(e.target.value) : undefined
                  )
                }
                min={1}
                max={6}
                disabled={isLoading}
              />
            </div>
            <div className="form-group">
              <label htmlFor="place_number">着順</label>
              <input
                id="place_number"
                type="number"
                placeholder="1-6"
                value={formData.place_number || ""}
                onChange={(e) =>
                  handleInputChange(
                    "place_number",
                    e.target.value ? Number(e.target.value) : undefined
                  )
                }
                min={1}
                max={6}
                disabled={isLoading}
              />
            </div>
            <div className="form-group">
              <label htmlFor="racer_number">選手登録番号</label>
              <input
                id="racer_number"
                type="number"
                placeholder="例: 3960"
                value={formData.racer_number || ""}
                onChange={(e) =>
                  handleInputChange(
                    "racer_number",
                    e.target.value ? Number(e.target.value) : undefined
                  )
                }
                min={0}
                disabled={isLoading}
              />
            </div>
          </div>
        </div>
      )}

      {/* 検索ボタン */}
      <button
        type="submit"
        disabled={isLoading}
        className="search-button"
        style={{ marginTop: "20px" }}
      >
        {isLoading ? "検索中..." : "検索"}
      </button>
    </form>
  );
};
