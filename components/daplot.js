import { Stack, useMantineColorScheme } from '@mantine/core'
import Plot from "react-plotly.js"

export default function Daplot(props) {
    const { colorScheme } = useMantineColorScheme()
    const dark = colorScheme === "dark"

    const trace1 = {
        name: 'k1',
        x: [0, 1, 2],
        y: [10, 11, 12],
        type: 'scatter'
    }

    const trace2 = {
        name: 'k2',
        x: [0, 1, 2],
        y: [110, 120, 100],
        xaxis: 'x2',
        yaxis: 'y2',
        type: 'scatter'
    }

    const trace3 = {
        name: 'k3',
        x: [0, 1, 2],
        y: [1100, 1000, 1200],
        xaxis: 'x3',
        yaxis: 'y3',
        type: 'scatter'
    }

    const trace4 = {
        name: 'k4',
        x: [0, 1, 2],
        y: [1200, 1000, 1100],
        xaxis: 'x4',
        yaxis: 'y4',
        type: 'scatter'
    }

    const trace5 = {
        name: 'k5',
        x: [0, 1, 2],
        y: [1100, 1000, 1200],
        xaxis: 'x5',
        yaxis: 'y5',
        type: 'scatter'
    }

    const trace6 = {
        name: 'k6',
        x: [0, 1, 2],
        y: [1000, 1200, 1100],
        xaxis: 'x6',
        yaxis: 'y6',
        type: 'scatter'
    }

    return <Stack
        align={'stretch'}
        justify={'center'}
        style={{ padding: 8 }}
    >
        <Plot
            data={[trace1, [], trace2, trace3, trace4, trace5, trace6]}
            layout={{
                paper_bgcolor: dark ? "#25262B" : "#DEE2E6",
                plot_bgcolor: dark ? "#25262B" : "#DEE2E6",
                font: {
                    family: 'Courier New, monospace',
                    size: 16,
                    color: '#7f7f7f'
                },
                grid: {
                    rows: 6,
                    columns: 1,
                    pattern: 'independent',
                    roworder: 'top to bottom'
                },
                margin: {
                    b: 25,
                    t: 30,
                    l: 0,
                    r: 0
                },
                legend: { orientation: 'h' }
            }}
            config={{ displaylogo: false, responsive: true }}
        />
    </Stack>
}
