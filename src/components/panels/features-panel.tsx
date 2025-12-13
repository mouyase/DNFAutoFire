import { GroupBox } from "@/components/ui"

interface Feature {
  id: string
  label: string
  enabled?: boolean
}

interface FeaturesPanelProps {
  features?: Feature[]
  onToggleFeature?: (id: string, enabled: boolean) => void
}

const DEFAULT_FEATURES: Feature[] = [
  { id: "flat-x", label: "原地平X(Beta)" },
  { id: "traveler-meteor", label: "旅人自动流星" },
  { id: "battle-mage-pattern", label: "战法自动炫纹" },
  { id: "emperor-sword-delay", label: "太宗帝剑延迟" },
]

/**
 * 其他功能面板
 */
export function FeaturesPanel({
  features = DEFAULT_FEATURES,
  onToggleFeature,
}: FeaturesPanelProps) {
  return (
    <GroupBox title="其他功能" className="h-full">
      <div className="flex flex-col gap-1.5">
        {features.map((feature) => (
          <label
            key={feature.id}
            className="flex items-center gap-2 text-xs text-text-link cursor-pointer hover:underline"
          >
            <input
              type="checkbox"
              className="w-3 h-3"
              checked={feature.enabled}
              onChange={(e) => onToggleFeature?.(feature.id, e.target.checked)}
            />
            {feature.label}
          </label>
        ))}
      </div>
    </GroupBox>
  )
}
