import { CharacterStats, ResolvedCalculatorResult, SortResultsSerde, SortResultsSerdeBase, Element } from "@/bindings.gen"
import { CellContext, ColumnDef, SortingState, createColumnHelper, flexRender, getCoreRowModel, getSortedRowModel, useReactTable } from "@tanstack/react-table"
import { Table, TableHeader, TableRow, TableHead, TableBody, TableCell } from "../ui/table"
import { useMemo, useState } from "react"
import { Button } from "../ui/button"
import { cn } from "@/utils"
import { ScrollArea, ScrollBar } from "../ui/scroll-area"
import { ArrowDown10 } from "lucide-react"

// type OptimizerData = CharacterStats

const ElementIndex: Record<Element, number> = {
    Physical:  0,
    Fire:      1,
    Ice:       2,
    Thunder:   3, // Lightning
    Wind:      4,
    Quantum:   5,
    Imaginary: 6,
}

const helper = createColumnHelper<ResolvedCalculatorResult>()

type Context = CellContext<ResolvedCalculatorResult, number>
const RoundedFormatter = Intl.NumberFormat(undefined, { maximumFractionDigits: 0 })
const DecimalFormatter = Intl.NumberFormat(undefined, { maximumFractionDigits: 1 })
const PercentFormatter = Intl.NumberFormat(undefined, { style: "percent", maximumFractionDigits: 1 })

const roundCell = (context: Context) => RoundedFormatter.format(context.getValue())
const decimalCell = (context: Context) => DecimalFormatter.format(context.getValue())
const percentCell = (context: Context) => PercentFormatter.format(context.getValue())

export function OptimizerTable({ data: allSorts, className, ...props }: React.HTMLAttributes<HTMLDivElement> & { data?: SortResultsSerde }) {
    // const columns: ColumnDef<ResolvedCalculatorResult>[] = [
    //     {
    //         accessorKey: ""
    //     }
    // ]
    const statType = 1 as 0 | 1

    const [activeColumn, setActiveColumn] = useState("atk")

    const columnMap = useMemo(() => {
        const map = new Map<string, ResolvedCalculatorResult[]>()
        if (!allSorts) return map

        const src = statType === 0 ? allSorts.base : allSorts.combat
        for (const key of Object.keys(src) as (keyof SortResultsSerdeBase)[]) {
            map.set(key, src[key])
        }

        for (const [name, values] of allSorts.cols) {
            map.set(name, values)
        }

        return map
    }, [allSorts, statType])

    const sum = statType === 1 ? "Σ " : ""
    const data = useMemo(() => columnMap.get(activeColumn) ?? [], [columnMap, activeColumn])
    const extraCols = useMemo(() => (data?.[0]?.cols ?? []).map((c, i) => [c[0], i] as const), [data])
    const columns = useMemo(() => [
        helper.accessor(row => row.calculated_stats[statType].atk, { id: "atk", header: sum+"ATK", cell: roundCell   }),
        helper.accessor(row => row.calculated_stats[statType].def, { id: "def", header: sum+"DEF", cell: roundCell   }),
        helper.accessor(row => row.calculated_stats[statType].hp,  { id: "hp" , header: sum+"HP" , cell: roundCell   }),
        helper.accessor(row => row.calculated_stats[statType].spd, { id: "spd", header: sum+"SPD", cell: decimalCell }),

        helper.accessor(row => row.calculated_stats[statType].crit_rate,       { id: "crit_rate"      , header: sum+"CR" , cell: percentCell }),
        helper.accessor(row => row.calculated_stats[statType].crit_dmg,        { id: "crit_dmg"       , header: sum+"CD" , cell: percentCell }),
        helper.accessor(row => row.calculated_stats[statType].effect_hit_rate, { id: "effect_hit_rate", header: sum+"EHR", cell: percentCell }),
        helper.accessor(row => row.calculated_stats[statType].effect_res,      { id: "effect_res"     , header: sum+"RES", cell: percentCell }),
        helper.accessor(row => row.calculated_stats[statType].break_effect,    { id: "break_effect"   , header: sum+"BE" , cell: percentCell }),
        helper.accessor(row => row.calculated_stats[statType].energy_recharge, { id: "energy_recharge", header: sum+"ERR", cell: percentCell }),
        helper.accessor(row => row.calculated_stats[statType].outgoing_healing_boost, { id: "outgoing_healing_boost", header: sum+"HEAL", cell: percentCell }),
        helper.accessor(row => row.calculated_stats[statType].elemental_dmg_boost[ElementIndex[allSorts?.effective_element ?? "Physical"]], { id: "elemental_dmg_boost", header: sum+"ELEM", cell: percentCell }),

        ...extraCols.map(([col, i]) => helper.accessor(row => row.cols[i][1], { id: col, header: col, cell: decimalCell })),
    ], [statType, extraCols]) // TODO: Shallow compare extraCols by spreading? maybe? if it's slow

    const table = useReactTable({
        columns,
        data,
        getCoreRowModel: getCoreRowModel(),
    })

    return (
        <div className={cn("rounded-md border", className)} {...props}>
          <Table>
            <TableHeader>
              {table.getHeaderGroups().map((headerGroup) => (
                <TableRow key={headerGroup.id}>
                  {headerGroup.headers.map((header) => {
                    return (
                      <TableHead key={header.id} className="w-[100px]">
                        {header.isPlaceholder
                          ? null
                          : <Button
                                variant="ghost"
                                size="sm"
                                onClick={() => setActiveColumn(header.column.id)}
                                // disabled={header.column.id === activeColumn}
                                className={cn(activeColumn !== header.id && "mx-[12px]")}
                            >
                                <ArrowDown10 size={16} className={cn("mr-[8px]", activeColumn !== header.id && "hidden")}/>
                                {flexRender(
                                    header.column.columnDef.header,
                                    header.getContext()
                                )}
                                {/* <ArrowDown10 size={16} className={"mr-2 opacity-0"}/> */}
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
                      <TableCell key={cell.id} className="text-center w-[100px]">
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
