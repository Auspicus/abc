import styles from './Grid.module.css'

const Grid = ({ columns = [], rows = [], }) => (
  <table className={styles.Grid}>
    <thead>
      <tr>
        {columns.map((col, i) => (
          <th key={i} className={styles.GridHeadingCell}>{col}</th>
        ))}
      </tr>
    </thead>
    <tbody>
      {rows.map(row => (
        <tr>
          {row.map((val, i) => (
            <td key={row[0] + i} className={styles.GridDataCell}>{val}</td>
          ))}
        </tr>
      ))}
    </tbody>
  </table>
)

export default Grid