export interface Parameter {
  name: string;
  type: string;
  description: string;
  required: boolean;
}

export interface CommandDocCardProps {
  title: string;
  commandName: string;
  functionName?: string;
  description: string;
  parameters: Parameter[];
  returnType: string;
  example?: string;
  implementation?: string;
}

export function CommandDocCard({
  title,
  commandName,
  functionName,
  description,
  parameters,
  returnType,
  example,
  implementation,
}: CommandDocCardProps) {
  return (
    <div className="command-card">
      <div className="command-header">
        <h3 className="command-title">{title}</h3>
      </div>

      <div className="command-meta">
        <span className="command-name-label">コマンド: <code>{commandName}</code></span>
        {functionName && (
          <>
            <span className="meta-separator">•</span>
            <span className="function-name-label">Hook: <code>{functionName}</code></span>
          </>
        )}
      </div>

      {implementation && (
        <div className="implementation-path">
          <small>実装: {implementation}</small>
        </div>
      )}

      <p className="command-description">{description}</p>

      {parameters.length > 0 && (
        <div className="params-section">
          <h4>パラメータ</h4>
          <table className="params-table">
            <thead>
              <tr>
                <th>名前</th>
                <th>型</th>
                <th>必須</th>
                <th>説明</th>
              </tr>
            </thead>
            <tbody>
              {parameters.map((param, index) => (
                <tr key={index}>
                  <td><code>{param.name}</code></td>
                  <td><code>{param.type}</code></td>
                  <td>{param.required ? "✓" : "-"}</td>
                  <td>{param.description}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      )}

      <div className="return-type-section">
        <h4>戻り値</h4>
        <code>{returnType}</code>
      </div>

      {example && (
        <div className="example-section">
          <h4>使用例</h4>
          <pre className="code-example"><code>{example}</code></pre>
        </div>
      )}
    </div>
  );
}
