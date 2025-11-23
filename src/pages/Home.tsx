import { Link } from "react-router-dom";
import "./Home.css";

function Home() {
  return (
    <div className="home-container">
      <div className="home-content">
        <h1 className="home-title">競艇スクレイピングツール</h1>
        <p className="home-description">
          競艇のレース情報、オッズ情報を取得するためのツールです。
          <br />
          以下から機能を選択してください。
        </p>

        <div className="home-menu">
          <Link to="/scraping" className="menu-card">
            <div className="card-icon">📊</div>
            <h2>スクレイピングツール</h2>
            <p>競艇日和からレースデータやオッズ情報を取得します</p>
            <ul className="feature-list">
              <li>レース情報取得</li>
              <li>単勝・複勝オッズ取得</li>
              <li>一括データ取得</li>
            </ul>
          </Link>

          <div className="menu-card disabled">
            <div className="card-icon">🚧</div>
            <h2>新機能（準備中）</h2>
            <p>今後追加予定の機能がここに表示されます</p>
          </div>
        </div>
      </div>
    </div>
  );
}

export default Home;
