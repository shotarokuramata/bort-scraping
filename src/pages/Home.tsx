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

          <Link to="/open-api" className="menu-card">
            <div className="card-icon">🗄️</div>
            <h2>Open API データ管理</h2>
            <p>Boatrace Open APIからデータを取得してデータベースに保存します</p>
            <ul className="feature-list">
              <li>Previews / Results / Programs 取得</li>
              <li>SQLiteデータベース保存</li>
              <li>CSV形式でエクスポート</li>
            </ul>
          </Link>

          <Link to="/high-payout-search" className="menu-card">
            <div className="card-icon">💰</div>
            <h2>高配当レース検索</h2>
            <p>高配当のレース結果を検索して分析します</p>
            <ul className="feature-list">
              <li>3連単・2連単・単勝・複勝の高配当検索</li>
              <li>配当統計情報の表示</li>
              <li>レース条件と配当の分析</li>
            </ul>
          </Link>
        </div>
      </div>
    </div>
  );
}

export default Home;
