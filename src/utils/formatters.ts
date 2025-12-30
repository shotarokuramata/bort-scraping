// 日付フォーマット YYYYMMDD -> YYYY/MM/DD
export const formatRaceDate = (dateStr: string): string => {
  if (dateStr.length !== 8) return dateStr;
  return `${dateStr.slice(0, 4)}/${dateStr.slice(4, 6)}/${dateStr.slice(6, 8)}`;
};

// 級別名取得
export const getRacerClassName = (classNum?: number): string => {
  const classMap: { [key: number]: string } = {
    1: "A1",
    2: "A2",
    3: "B1",
    4: "B2",
  };
  return classNum ? classMap[classNum] || "-" : "-";
};

// グレード名取得
export const getRaceGradeName = (gradeNum?: number): string => {
  const gradeMap: { [key: number]: string } = {
    1: "SG",
    2: "G1",
    3: "G2",
    4: "G3",
    5: "一般",
  };
  return gradeNum ? gradeMap[gradeNum] || "-" : "-";
};

// 配当フォーマット
export const formatPayout = (payout?: number): string => {
  return payout ? `¥${payout.toLocaleString("ja-JP")}` : "-";
};
