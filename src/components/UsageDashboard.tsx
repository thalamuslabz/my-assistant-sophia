import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";

interface UsageStats {
  provider: string;
  total_requests: number;
  total_tokens: number;
  total_cost_usd: number;
  period_start: string;
  period_end: string;
}

export function UsageDashboard() {
  const [stats, setStats] = useState<UsageStats[]>([]);
  const [totalCost, setTotalCost] = useState<number>(0);
  const [period, setPeriod] = useState<number>(7); // Default: 7 days
  const [loading, setLoading] = useState(false);

  const loadStats = async () => {
    setLoading(true);
    try {
      const usageStats = await invoke<UsageStats[]>("get_usage_stats", { days: period });
      const cost = await invoke<number>("get_total_cost", { days: period });
      setStats(usageStats);
      setTotalCost(cost);
    } catch (err) {
      console.error("Failed to load usage stats:", err);
    } finally {
      setLoading(false);
    }
  };

  useEffect(() => {
    loadStats();
  }, [period]);

  const formatCost = (cost: number) => {
    return new Intl.NumberFormat('en-US', {
      style: 'currency',
      currency: 'USD',
      minimumFractionDigits: 4,
    }).format(cost);
  };

  const formatNumber = (num: number) => {
    return new Intl.NumberFormat('en-US').format(num);
  };

  return (
    <div className="usage-dashboard">
      <div className="usage-header">
        <h2>Usage & Cost Tracking</h2>
        <select value={period} onChange={(e) => setPeriod(Number(e.target.value))}>
          <option value={1}>Last 24 hours</option>
          <option value={7}>Last 7 days</option>
          <option value={30}>Last 30 days</option>
          <option value={90}>Last 90 days</option>
        </select>
      </div>

      {loading ? (
        <div className="usage-loading">Loading stats...</div>
      ) : (
        <>
          <div className="usage-summary">
            <div className="usage-card total-cost">
              <h3>Total Cost</h3>
              <div className="usage-value">{formatCost(totalCost)}</div>
              <div className="usage-label">Last {period} days</div>
            </div>
          </div>

          <div className="usage-table">
            <table>
              <thead>
                <tr>
                  <th>Provider</th>
                  <th>Requests</th>
                  <th>Tokens</th>
                  <th>Cost</th>
                </tr>
              </thead>
              <tbody>
                {stats.length === 0 ? (
                  <tr>
                    <td colSpan={4} className="no-data">
                      No usage data for this period
                    </td>
                  </tr>
                ) : (
                  stats.map((stat) => (
                    <tr key={stat.provider}>
                      <td className="provider-name">{stat.provider}</td>
                      <td>{formatNumber(stat.total_requests)}</td>
                      <td>{formatNumber(stat.total_tokens)}</td>
                      <td>{formatCost(stat.total_cost_usd)}</td>
                    </tr>
                  ))
                )}
              </tbody>
            </table>
          </div>

          <button onClick={loadStats} className="refresh-btn">
            Refresh Stats
          </button>
        </>
      )}
    </div>
  );
}
