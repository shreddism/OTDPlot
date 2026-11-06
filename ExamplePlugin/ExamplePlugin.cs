using System;
using System.Numerics;
using OpenTabletDriver.Plugin.Attributes;
using OpenTabletDriver.Plugin.Output;
using OpenTabletDriver.Plugin.Tablet;
using OpenTabletDriver.Plugin.Timing;       

namespace OTDPlot
{
    [PluginName("OTDPlot.ExamplePlugin")]
    public class ExamplePlugin : IPositionedPipelineElement<IDeviceReport>
    {
        public ExamplePlugin() : base()
        {
        }

        public PipelinePosition Position => PipelinePosition.PreTransform;

        public event Action<IDeviceReport> Emit;

        public void Consume(IDeviceReport value)
        {
            if (value is ITabletReport report)
            {
                tick++;
                int action = tick % 2 == 0 ? 2 : 1;             // should always update, draw every other report because of high report rate
                Plot("v", report.Position - lastPosition, 0);
                Plot("a", report.Position - tabletCenter, action);  // update should only be on the last
                lastPosition = report.Position;
            }
            Emit?.Invoke(value);
        }

        int tick;
        Vector2 lastPosition;
        Vector2 tabletCenter;

        public void Plot(string name, Vector2 data, int act) 
        {
            Console.Write(name + "x");
            Console.WriteLine(data.X);
            Console.Write(name + "y");
            Console.WriteLine(data.Y * -1);
            if (act > 0) {
                Console.WriteLine("xx");
                if (act > 1) {
                    Console.WriteLine("dd");
                }
            }
        }

        [TabletReference]
        public TabletReference Tablet
        {
            set 
            {
                var area = value.Properties.Specifications.Digitizer;
                tabletCenter = new Vector2(area.MaxX * 0.5f, area.MaxY * 0.5f);
            }
        }
    }
}