import { JingliuConfig } from "@/bindings.gen";
import { Combobox } from "@/components/ui/combobox";
import { JingliuKit } from "@/kits/characters/jingliu";
import { createFileRoute } from "@tanstack/react-router";
import { useState } from "react";

export const Route = createFileRoute("/")({
    component: Index,
});

enum Characters {
    Jingliu = "jingliu",
    Xueyi = "xueyi",
    Sparkle = "sparkle",
}

function Index() {
    const [character, setCharacter] = useState<string>(Characters.Jingliu);

    const [kit, setKit] = useState<JingliuConfig>({
        enhanced_state: true,
        e1_crit_dmg: true,
        e2_skill_buff: true,
        hp_drain_pct: 1.0,
    });

    return (
        <div className="p-2">
            <h3>Welcome Home!</h3>
            <Combobox
                value={character}
                onChange={setCharacter}
                deselectable={false}
                options={[
                    { value: Characters.Jingliu, label: "Jingliu" },
                    { value: Characters.Xueyi, label: "Xueyi" },
                    { value: Characters.Sparkle, label: "Sparkle" },
                ]}
            />

            <div className="w-[300px] mt-4 border p-4 rounded-md bg-card">
                <JingliuKit
                    characterState={{
                        level: 80,
                        eidolon: 6,
                        ascension: 6,
                        skills: {
                            basic: 7 - 1,
                            skill: 12 - 1,
                            ult: 12 - 1,
                            talent: 12 - 1,
                        },
                        traces: {
                            ability_1: true,
                            ability_2: true,
                            ability_3: true,
                            stat_1: true,
                            stat_2: true,
                            stat_3: true,
                            stat_4: true,
                            stat_5: true,
                            stat_6: true,
                            stat_7: true,
                            stat_8: true,
                            stat_9: true,
                            stat_10: true,
                        },
                    }}
                    value={kit}
                    onChange={(value) => {
                        console.log(value);
                        setKit(value);
                    }}
                />
            </div>
        </div>
    );
}
