# XHSTT Genetic Algorithm

## XHSTT Problem Instances
Timetabling problems in the XHSTT XML format can be downloaded
[here](https://www.utwente.nl/en/eemcs/dmmp/hstt/archives/).

Files are available as collections (containing multiple problem instances)
and as single-instance files. Currently, single-instance files are preferred,
as they also come with the known solutions.

To get started with implementing a scheduler, the artificial problem instances
have been chosen, so it's not needed to implement a whole lot of constraints.

# XHSTT
## Datastructure
```sh
HighSchoolTimetableArchive [+Id]
    + MetaData
        Name
        Contributor
        Date
        Description
        + Remarks

    + Instances
        * Instance [Id]
            MetaData
                Name
                Contributor
                Date
                Country
                Description
                + Remarks

            Times
                + TimeGroups
                    * Week [Id]
                        Name
                        + TimeGroup ???

                    * Day [Id]
                        Name
                        + TimeGroup ???

                    * TimeGroup [Id]
                        Name

                * Time [Id]
                    Name
                    + Week [Reference]
                    + Day [Reference]
                    + TimeGroups
                        * TimeGroup [Reference]

            Resources
                + ResourceTypes
                    * ResourceType [Id]
                        Name

                + ResourceGroups
                    * ResourceGroup [Id]
                        Name
                        ResourceType [Reference]

                * Resource [Id]
                    Name
                    ResourceType [Reference]
                    + ResourceGroups
                        * ResourceGroup [Reference]

            Events
                + EventGroups
                    * Course [Id]
                        Name
                        + EventGroup ??? (similar to week, day and timegroup)

                    * EventGroup [Id]
                        Name

                * Event [Id] [+Color]
                    Name
                    Duration (int >= 1)
                    + Workload
                    + Course [Reference]
                    + Time [Reference]
                    + Resources
                        * Resource [+Reference]
                            + Role (body = value)
                            + ResourceType [Reference] (not documented well)
                            + Workload

                    + ResourceGroups
                        * ResourceGroup [Reference]

                    + EventGroups
                        * EventGroup [Reference]

            Constraints
                Name
                Required
                ...

                AppliesTo (events; both)
                    + EventGroups
                        * EventGroup [Reference]

                    + Events
                        * Event [Reference]

                AppliesTo (events; groups only)
                    EventGroups
                        * EventGroup [Reference]


                AppliesTo (events; pairs)
                    EventPairs
                        * EventPair
                            FirstEvent [Reference]
                            SecondEvent [Reference]
                            + MinSeparation
                            + MaxSeparation

                AppliesTo (resources; both)
                    + ResourceGroups
                        * ResourceGroup [Reference]

                    + Resources
                        * Resource [Reference]

    + SolutionGroups
        * SolutionGroup [Id]
            MetaData
                Contributor
                Date
                Description
                + Publication
                + Remarks

            * Solution [Reference]
                + Description
                + RunningTime
                + Events
                    * Event [Reference]
                        + Duration
                        + Time [Reference]
                        + Ressources
                            * Resource [Reference]
                                Role
                + Report
                    InfeasibilityValue
                    ObjectiveValue
                    + Resources
                        * Resource [Reference]
                            * Constraint [Reference]
                                Cost
                                + Description
                    + Events
                        * Event [Reference]
                            * Constraint [Reference]
                                Cost
                                + Description

                    + EventGroups
                        * EventGroup [Reference]
                            * Constraint [Reference]
                                Cost
                                + Description

```

