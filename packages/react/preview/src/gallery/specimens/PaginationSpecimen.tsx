import { useState } from "react";
import { Pagination } from "@poodle/react";
import { SpecimenGroup } from "../SpecimenGroup";
import { SpecimenLayout } from "../SpecimenLayout";

const totalItems = 248;

export function PaginationSpecimen() {
  const [page1, setPage1] = useState(1);
  const [page2, setPage2] = useState(5);
  const [page3, setPage3] = useState(2);
  const [limit3, setLimit3] = useState(25);
  const [page4, setPage4] = useState(1);

  const totalPagesForLimit3 = Math.ceil(totalItems / limit3);

  return (
    <SpecimenLayout
      sizes={(size) => <Pagination currentPage={1} totalPages={10} size={size} ariaLabel={`${size} pagination`} />}
      densities={(density) => <Pagination totalPages={10} density={density} />}
    >
      <SpecimenGroup label="Default">
        <Pagination
          currentPage={page1}
          totalPages={10}
          ariaLabel="Results pagination"
          onPageChange={(page) => setPage1(page)}
        />
        <p>
          Page <strong>{page1}</strong> of 10
        </p>
      </SpecimenGroup>

      <SpecimenGroup label="Middle of range">
        <Pagination
          currentPage={page2}
          totalPages={20}
          siblingCount={2}
          ariaLabel="Extended pagination"
          onPageChange={(page) => setPage2(page)}
        />
        <p>
          Page <strong>{page2}</strong> of 20
        </p>
      </SpecimenGroup>

      <SpecimenGroup label="Few pages">
        <Pagination currentPage={2} totalPages={3} ariaLabel="Short pagination" />
      </SpecimenGroup>

      <SpecimenGroup label="Simple variant with info and page size">
        <Pagination
          page={page3}
          limit={limit3}
          total={totalItems}
          variant="simple"
          showLimitSelector
          limitOptions={[10, 25, 50, 100]}
          ariaLabel="Simple pagination"
          onPageChange={(page) => setPage3(page)}
          onLimitChange={(limit) => {
            setLimit3(limit);
            setPage3(1);
          }}
        />
        <p>
          Page <strong>{page3}</strong> of {totalPagesForLimit3} with <strong>{limit3}</strong> per page
        </p>
      </SpecimenGroup>

      <SpecimenGroup label="Full variant">
        <Pagination
          page={page4}
          limit={20}
          total={140}
          variant="full"
          ariaLabel="Full pagination"
          onPageChange={(page) => setPage4(page)}
        />
        <p>
          Page <strong>{page4}</strong> of 7
        </p>
      </SpecimenGroup>

      <SpecimenGroup label="With container chrome">
        <Pagination currentPage={1} totalPages={10} chrome ariaLabel="Pagination with chrome" />
      </SpecimenGroup>
    </SpecimenLayout>
  );
}
