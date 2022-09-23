import { useEffect, useState } from 'react'
import { Stack, useMantineColorScheme } from '@mantine/core'
import Plot from "react-plotly.js"


export default function Daplot() {
    const [dt1, setDt1] = useState([0])
    const [dt2, setDt2] = useState([0])
    const [dt3, setDt3] = useState([0])
    const [dt4, setDt4] = useState([0])
    const [dt5, setDt5] = useState([0])
    const [dt6, setDt6] = useState([0])

    const { colorScheme } = useMantineColorScheme()
    const dark = colorScheme === "dark"

    useEffect(() => wsInit(), [])

    const wsInit = () => {
        let socket = new WebSocket("ws://localhost:9001")

        socket.onmessage = function (e) {
            const data = e.data
            const payload = data.split(",")

            if (!payload.includes('SEQ') && !payload.includes('ENDSEQ')) {
                payload.splice(-1)

                setDt1(old => [...old, payload[0]])
                setDt2(old => [...old, payload[1]])
                setDt3(old => [...old, payload[2]])
                setDt4(old => [...old, payload[3]])
                setDt5(old => [...old, payload[4]])
                setDt6(old => [...old, payload[5]])
            }
        }

        socket.onclose = function (e) {
            console.log(e)
        }

        socket.onerror = function (e) {
            console.log(e)
        }
    }

    const trace1 = {
        name: 'k1',
        y: dt1,
        xaxis: 'x1',
        yaxis: 'y1',
        type: 'scatter'
    }

    const trace2 = {
        name: 'k2',
        y: dt2,
        xaxis: 'x2',
        yaxis: 'y2',
        type: 'scatter'
    }

    const trace3 = {
        name: 'k3',
        y: dt3,
        xaxis: 'x3',
        yaxis: 'y3',
        type: 'scatter'
    }

    const trace4 = {
        name: 'k4',
        y: dt4,
        xaxis: 'x4',
        yaxis: 'y4',
        type: 'scatter'
    }

    const trace5 = {
        name: 'k5',
        y: dt5,
        xaxis: 'x5',
        yaxis: 'y5',
        type: 'scatter'
    }

    const trace6 = {
        name: 'k6',
        y: dt6,
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
            data={[trace1, trace2, trace3, trace4, trace5, trace6]}
            layout={{
                height: 560,
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
