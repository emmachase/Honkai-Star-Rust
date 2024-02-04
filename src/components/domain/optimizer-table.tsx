import { CharacterStats, ResolvedCalculatorResult, SortResultsSerde } from "@/bindings.gen"
import { CellContext, ColumnDef, SortingState, createColumnHelper, flexRender, getCoreRowModel, getSortedRowModel, useReactTable } from "@tanstack/react-table"
import { Table, TableHeader, TableRow, TableHead, TableBody, TableCell } from "../ui/table"
import { useMemo, useState } from "react"
import { Button } from "../ui/button"

// type OptimizerData = CharacterStats

const helper = createColumnHelper<ResolvedCalculatorResult>()

type Context = CellContext<ResolvedCalculatorResult, number>
const RoundedFormatter = Intl.NumberFormat(undefined, { maximumFractionDigits: 0 })
const DecimalFormatter = Intl.NumberFormat(undefined, { maximumFractionDigits: 1 })
const PercentFormatter = Intl.NumberFormat(undefined, { style: "percent", maximumFractionDigits: 1 })

const roundCell = (context: Context) => RoundedFormatter.format(context.getValue())
const decimalCell = (context: Context) => DecimalFormatter.format(context.getValue())
const percentCell = (context: Context) => PercentFormatter.format(context.getValue())

export function OptimizerTable({ data: allSorts }: { data?: SortResultsSerde }) {
    // const columns: ColumnDef<ResolvedCalculatorResult>[] = [
    //     {
    //         accessorKey: ""
    //     }
    // ]
    const statType: 0 | 1 = 1

    const columnMap = useMemo(() => {
        const map = new Map<string, ResolvedCalculatorResult[]>()
        if (!allSorts) return map

        for (const key of Object.keys(allSorts) as (keyof SortResultsSerde)[]) {
            if (key !== "cols") {
                map.set(key, allSorts[key])
            } else {
                for (const [name, values] of allSorts.cols) {
                    map.set(name, values)
                }
            }
        }

        return map
    }, [allSorts])
    const [activeColumn, setActiveColumn] = useState("atk")

    const data = columnMap.get(activeColumn) ?? []

    const extraCols = useMemo(() =>
        (data?.[0]?.cols ?? []).map((c, i) => [c[0], i] as const)
    , [data])

    const columns = useMemo(() => [
        helper.accessor(row => row.calculated_stats[statType].atk, { id: "atk", header: "ATK", cell: roundCell   }),
        helper.accessor(row => row.calculated_stats[statType].def, { id: "def", header: "DEF", cell: roundCell   }),
        helper.accessor(row => row.calculated_stats[statType].hp,  { id: "hp" , header: "HP" , cell: roundCell   }),
        helper.accessor(row => row.calculated_stats[statType].spd, { id: "spd", header: "SPD", cell: decimalCell }),

        helper.accessor(row => row.calculated_stats[statType].crit_rate,       { id: "crit_rate"      , header: "CR" , cell: percentCell }),
        helper.accessor(row => row.calculated_stats[statType].crit_dmg,        { id: "crit_dmg"       , header: "CD" , cell: percentCell }),
        helper.accessor(row => row.calculated_stats[statType].effect_hit_rate, { id: "effect_hit_rate", header: "EHR", cell: percentCell }),
        helper.accessor(row => row.calculated_stats[statType].effect_res,      { id: "effect_res"     , header: "RES", cell: percentCell }),
        helper.accessor(row => row.calculated_stats[statType].break_effect,    { id: "break_effect"   , header: "BE" , cell: percentCell }),
        helper.accessor(row => row.calculated_stats[statType].energy_recharge, { id: "energy_recharge", header: "ERR", cell: percentCell }),
        helper.accessor(row => row.calculated_stats[statType].outgoing_healing_boost, { id: "outgoing_healing_boost", header: "HEAL", cell: percentCell }),
        helper.accessor(row => row.calculated_stats[statType].elemental_dmg_bonus,    { id: "elemental_dmg_bonus"   , header: "ELEM", cell: percentCell }),

        ...extraCols.map(([col, i]) => helper.accessor(row => row.cols[i][1], { id: col, header: col, cell: decimalCell })),
    ], [statType, extraCols]) // TODO: Shallow compare extraCols by spreading? maybe? if it's slow

    const table = useReactTable({
        columns,
        data,
        getCoreRowModel: getCoreRowModel(),
    })

    return (
        <div className="rounded-md border">
          <Table>
            <TableHeader>
              {table.getHeaderGroups().map((headerGroup) => (
                <TableRow key={headerGroup.id}>
                  {headerGroup.headers.map((header) => {
                    return (
                      <TableHead key={header.id}>
                        {header.isPlaceholder
                          ? null
                          : <Button
                                variant="ghost"
                                onClick={() => setActiveColumn(header.column.id)}
                            >{flexRender(
                              header.column.columnDef.header,
                              header.getContext()
                            )}
                            </Button>
                        }
                      </TableHead>
                    )
                  })}
                </TableRow>
              ))}
            </TableHeader>
            <TableBody>
              {table.getRowModel().rows?.length ? (
                table.getRowModel().rows.map((row) => (
                  <TableRow
                    key={row.id}
                    data-state={row.getIsSelected() && "selected"}
                  >
                    {row.getVisibleCells().map((cell) => (
                      <TableCell key={cell.id} className="text-center">
                        {flexRender(cell.column.columnDef.cell, cell.getContext())}
                      </TableCell>
                    ))}
                  </TableRow>
                ))
              ) : (
                <TableRow>
                  <TableCell colSpan={columns.length} className="h-24 text-center">
                    No results.
                  </TableCell>
                </TableRow>
              )}
            </TableBody>
          </Table>
        </div>
      )
}
